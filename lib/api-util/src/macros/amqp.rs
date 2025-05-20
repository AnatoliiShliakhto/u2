#[macro_export]
macro_rules! amqp_init {
    () => {{
        use ::api_util::{amqp::AMQPPool, env};
        use ::std::sync::Arc;
        
        let amqp = AMQPPool::init(
            env!("CARGO_PKG_NAME"),
            env::get_var_or_default("AMQP_URL", "amqp://root:root@rabbitmq:5672/%2f"),
        )
            .await?;
        
        Arc::new(amqp)
    }};
}
