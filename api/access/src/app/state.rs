use ::api_util::{Error, amqp::AMQPPool, amqp_init};
use ::std::sync::Arc;
use ::tokio::sync::OnceCell;

static APP: OnceCell<AppState> = OnceCell::const_new();

pub struct AppState {
    pub amqp: Arc<AMQPPool>,
}

pub async fn init_state() -> Result<&'static AppState, Error> {
    let state = AppState {
        amqp: amqp_init!(),
    };
    
    APP.set(state)
        .map_err(|_| Error::Unknown("Application state already set"))?;

    Ok(APP.get().unwrap())
}

pub fn get_state() -> &'static AppState {
    APP.get().expect("Application state is not set")
}