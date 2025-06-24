#[macro_export]
macro_rules! amqp_init {
    () => {
        amqp_init!(env!("CARGO_PKG_NAME"))
    };
    ($service_name:expr) => {{
        use ::api_util::{amqp::AMQPPool, env};
        use ::std::sync::Arc;
        
        let amqp_url = env::get_var_or_default("AMQP_URL", "amqp://root:root@rabbitmq:5672/%2f");
        let amqp_pool = AMQPPool::new(amqp_url).await?;
        
        Arc::new(amqp_pool)
    }};
}