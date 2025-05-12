use ::std::sync::Arc;
use ::api_util::amqp::AmqpPool;

pub struct AppState{
    pub amqp: Arc<AmqpPool>,
}

impl AppState {
    pub fn init(amqp: &Arc<AmqpPool>) -> Self {
        Self {
            amqp: amqp.clone(),
        }
    }
}