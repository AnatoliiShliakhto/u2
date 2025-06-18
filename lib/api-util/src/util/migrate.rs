use crate::{Error, env};
use ::std::{collections::BTreeMap, path::PathBuf, sync::LazyLock};
use ::surrealdb::{Surreal, engine::remote::ws::Client};
use ::tokio::fs::{self, DirEntry};

const MIGRATION_FOLDER_ENV: &str = "DATA_PATH";
const DEFAULT_MIGRATION_FOLDER: &str = "/etc/u2";
const MIGRATIONS_UP_PATTERN: &str = ".up.surql";
const MIGRATIONS_DOWN_PATTERN: &str = ".down.surql";
const DEFAULT_CURRENT_MIGRATION: i32 = -1;

static MIGRATIONS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env::get_var_or_default(
        MIGRATION_FOLDER_ENV,
        DEFAULT_MIGRATION_FOLDER,
    ))
    .join("migrations/")
});

pub trait MigrateExt {
    fn migrate_up(&self) -> impl Future<Output = Result<(), Error>>;
    fn migrate_down(&self) -> impl Future<Output = Result<(), Error>>;
    fn migrate_down_to(&self, idx: i32) -> impl Future<Output = Result<(), Error>>;
}

impl MigrateExt for Surreal<Client> {
    async fn migrate_up(&self) -> Result<(), Error> {
        let migrations_list = get_migration_list(MIGRATIONS_UP_PATTERN).await?;
        let current_migration = get_current_migration_or_default(self).await?;

        execute_pending_migrations(self, &migrations_list, current_migration).await
    }

    async fn migrate_down(&self) -> Result<(), Error> {
        let migrations_list = get_migration_list(MIGRATIONS_DOWN_PATTERN).await?;
        let current_migration = get_current_migration_or_default(self).await?;

        execute_rollback_migrations(self, &migrations_list, current_migration).await?;
        set_current_migration(self, DEFAULT_CURRENT_MIGRATION).await
    }

    async fn migrate_down_to(&self, target_index: i32) -> Result<(), Error> {
        let migrations_list = get_migration_list(MIGRATIONS_DOWN_PATTERN).await?;
        let current_migration = get_current_migration_or_default(self).await?;

        execute_targeted_rollback(self, &migrations_list, current_migration, target_index).await
    }
}

async fn get_current_migration_or_default(db: &Surreal<Client>) -> Result<i32, Error> {
    Ok(get_current_migration(db)
        .await?
        .unwrap_or(DEFAULT_CURRENT_MIGRATION))
}

async fn execute_pending_migrations(
    db: &Surreal<Client>,
    migrations: &BTreeMap<i32, PathBuf>,
    current_migration: i32,
) -> Result<(), Error> {
    for (idx, path) in migrations {
        if *idx > current_migration {
            execute_migration(db, *idx, path).await?;
        }
    }
    Ok(())
}

async fn execute_rollback_migrations(
    db: &Surreal<Client>,
    migrations: &BTreeMap<i32, PathBuf>,
    current_migration: i32,
) -> Result<(), Error> {
    for (idx, path) in migrations.iter().rev() {
        if *idx <= current_migration {
            execute_migration(db, *idx, path).await?;
            set_current_migration(db, *idx).await?;
        }
    }
    Ok(())
}

async fn execute_targeted_rollback(
    db: &Surreal<Client>,
    migrations: &BTreeMap<i32, PathBuf>,
    current_migration: i32,
    target_index: i32,
) -> Result<(), Error> {
    for (idx, path) in migrations.iter().rev() {
        if *idx <= current_migration && *idx > target_index {
            execute_migration(db, *idx, path).await?;
            set_current_migration(db, *idx - 1).await?;
        }
    }
    Ok(())
}

async fn execute_migration(db: &Surreal<Client>, idx: i32, path: &PathBuf) -> Result<(), Error> {
    let query = read_migration_file(path).await?;
    db.query(query.as_str()).await?.check()?;
    set_current_migration(db, idx).await
}

async fn read_migration_file(path: &PathBuf) -> Result<String, Error> {
    fs::read_to_string(path)
        .await
        .map_err(|_| Error::Unknown("failed to read migration file"))
}

async fn get_current_migration(db: &Surreal<Client>) -> Result<Option<i32>, Error> {
    let migration = db
        .query(include_str!("../../res/query/get_current_migration.surql"))
        .await?
        .take::<Option<i32>>(0)?;
    Ok(migration)
}

async fn set_current_migration(db: &Surreal<Client>, idx: i32) -> Result<(), Error> {
    db.query(include_str!("../../res/query/set_current_migration.surql"))
        .bind(("migration", idx))
        .await?
        .check()?;
    Ok(())
}

async fn get_migration_list(pattern: &str) -> Result<BTreeMap<i32, PathBuf>, Error> {
    let mut migrations_list = BTreeMap::new();
    let mut folder = match fs::read_dir(MIGRATIONS_PATH.clone()).await {
        Ok(folder) => folder,
        Err(_) => return Ok(migrations_list),
    };

    while let Ok(Some(child)) = folder.next_entry().await {
        if let Some((migration_index, path)) = parse_migration_entry(child, pattern).await {
            migrations_list.insert(migration_index, path);
        }
    }

    Ok(migrations_list)
}

async fn parse_migration_entry(entry: DirEntry, pattern: &str) -> Option<(i32, PathBuf)> {
    let meta = entry.metadata().await.ok()?;
    if !meta.is_file() {
        return None;
    }

    let filename = entry.file_name().into_string().ok()?;
    if !filename.ends_with(pattern) {
        return None;
    }

    let migration_number = filename.split('_').next()?;
    let migration_index = migration_number.parse::<i32>().ok()?;

    Some((migration_index, entry.path()))
}
