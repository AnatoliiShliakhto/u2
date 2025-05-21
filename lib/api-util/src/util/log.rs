use crate::amqp::{AMQPMessageOptions, AMQPPool, ExchangeKind};
use ::std::{io, sync::Arc, sync::Mutex};
use ::tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use ::tracing_subscriber::{
    EnvFilter, filter::LevelFilter, fmt::layer, layer::SubscriberExt, util::SubscriberInitExt,
};

pub use ::tracing::{debug, error, info, trace, warn};
pub use ::tracing_appender::rolling as rolling_appender;

pub struct LoggerWriter {
    amqp: Arc<AMQPPool>,
}

impl LoggerWriter {
    pub fn init(pool: &Arc<AMQPPool>) -> Self {
        Self {
            amqp: pool.clone(),
        }
    }
}

impl io::Write for LoggerWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let pool = self.amqp.clone();
        let buf_owned = buf.to_vec();

        tokio::spawn(async move {
            if let Err(err) = pool
                .send(
                    ExchangeKind::Topic,
                    "log.write",
                    AMQPMessageOptions::default().with_app_id(),
                    &buf_owned,
                )
                .await
            {
                eprintln!("'logger.write' sending AMQP message: {}", err);
            };
        });

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf).ok();
        Ok(())
    }
}

pub fn stdout_logger() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let stdout_layer = layer().compact();

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(env_filter)
        .init();
}

pub async fn amqp_logger(pool: &Arc<AMQPPool>) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let stdout_layer = layer().compact();
    let amqp_layer = layer()
        .compact()
        .with_ansi(false)
        .with_writer(Mutex::new(LoggerWriter::init(pool)));

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(amqp_layer)
        .with(env_filter)
        .init();
}

pub fn file_logger(path: &str, filename: &str) -> WorkerGuard {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(filename)
        .filename_suffix("log")
        .max_log_files(30)
        .build(path)
        .expect("failed to initialize rolling file appender");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = layer().compact();
    let file_layer = layer().compact().with_ansi(false).with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .with(env_filter)
        .init();

    guard
}
