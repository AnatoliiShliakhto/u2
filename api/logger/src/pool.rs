use ::api_util::logger::{
    error,
    rolling_appender::{RollingFileAppender, Rotation},
};
use ::std::{
    collections::HashMap,
    io::{Error, Write},
    sync::Arc,
};
use ::tokio::sync::Mutex;

pub struct Pool {
    path: String,
    pool: Mutex<HashMap<String, Arc<Mutex<RollingFileAppender>>>>,
}

impl Pool {
    pub fn new(path: String) -> Self {
        Self {
            path,
            pool: Mutex::new(HashMap::new()),
        }
    }

    pub async fn write(&self, filename: &str, data: &[u8]) -> Result<(), Error> {
        let file_rc = {
            let mut pool = self.pool.lock().await;

            if pool.contains_key(filename) {
                pool.get(filename).unwrap().clone()
            } else {
                let file_appender = RollingFileAppender::builder()
                    .filename_prefix(filename)
                    .filename_suffix("log")
                    .rotation(Rotation::DAILY)
                    .max_log_files(30)
                    .build(self.path.clone())
                    .expect("failed to initialize rolling file appender");
                let file = Arc::new(Mutex::new(file_appender));
                pool.insert(filename.to_string(), file.clone());
                file
            }
        };

        let mut file = file_rc.lock().await;
        if let Err(err) = file.write(data) {
            error!("failed to write to file: {}", err);
        }

        Ok(())
    }
}
