use crate::{Error, env};
use ::std::{collections::BTreeMap, path::PathBuf, sync::LazyLock};
use ::surrealdb::{Surreal, engine::remote::ws::Client};
use ::tokio::fs;

static MIGRATIONS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env::get_var_or_default("DATA_PATH", "/etc/u2")).join("migrations/")
});

pub trait MigrateExt {
    fn migrate_up(&self) -> impl Future<Output = Result<(), Error>>;
    fn migrate_down(&self) -> impl Future<Output = Result<(), Error>>;
    fn migrate_down_to(&self, idx: i32) -> impl Future<Output = Result<(), Error>>;
}

impl MigrateExt for Surreal<Client> {
    async fn migrate_up(&self) -> Result<(), Error> {
        let migrations_list = get_migration_list(".up.surql").await?;
        let current_migration = get_current_migration(self).await?.unwrap_or(-1);

        for (idx, path) in migrations_list {
            if idx > current_migration {
                migrate(self, idx, path).await?
            }
        }

        Ok(())
    }

    async fn migrate_down(&self) -> Result<(), Error> {
        let migrations_list = get_migration_list(".down.surql").await?;
        let current_migration = get_current_migration(self).await?.unwrap_or(-1);

        for (idx, path) in migrations_list.into_iter().rev() {
            if idx <= current_migration {
                migrate(self, idx, path).await?;
                set_current_migration(self, idx).await?
            }
        }

        set_current_migration(self, -1).await?;

        Ok(())
    }

    async fn migrate_down_to(&self, index: i32) -> Result<(), Error> {
        let migrations_list = get_migration_list(".down.surql").await?;
        let current_migration = get_current_migration(self).await?.unwrap_or(-1);

        for (idx, path) in migrations_list.into_iter().rev() {
            if idx <= current_migration && idx > index {
                migrate(self, idx, path).await?;
                set_current_migration(self, idx - 1).await?
            }
        }

        Ok(())
    }
}

async fn migrate(db: &Surreal<Client>, idx: i32, path: PathBuf) -> Result<(), Error> {
    let Ok(query) = fs::read_to_string(path).await else {
        Err(Error::Unknown("failed to read migration file"))?
    };

    db.query(query.as_str()).await?.check()?;
    set_current_migration(db, idx).await?;

    Ok(())
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
    let Ok(mut folder) = fs::read_dir(MIGRATIONS_PATH.clone()).await else {
        return Ok(migrations_list);
    };
    while let Ok(Some(child)) = folder.next_entry().await {
        let Ok(meta) = child.metadata().await else {
            continue;
        };
        if !meta.is_file() {
            continue;
        }
        let Ok(filename) = child.file_name().into_string() else {
            continue;
        };
        if !filename.ends_with(pattern) {
            continue;
        }

        let Some(num) = filename.split('_').next() else {
            continue;
        };
        let Ok(num) = num.parse::<i32>() else {
            continue;
        };

        migrations_list.insert(num, child.path());
    }

    Ok(migrations_list)
}
