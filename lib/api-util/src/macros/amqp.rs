#[macro_export]
macro_rules! amqp_init {
    () => {
        amqp_init!(env!("CARGO_PKG_NAME"))
    };
    ($service_name:expr) => {{
        use ::api_util::{amqp::AMQPPool, env};
        
        let amqp_url = env::get_var_or_default("AMQP_URL", "amqp://root:root@rabbitmq:5672/%2f");
        AMQPPool::new(amqp_url).await?
    }};
}