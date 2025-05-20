use super::ExchangeKind;

pub struct AMQPChannelOptions {
    pub(crate) exchange: ExchangeKind,
    pub(crate) routing_key: String,
    pub(crate) passive: bool,
    pub(crate) durable: bool,
    pub(crate) exclusive: bool,
    pub(crate) auto_delete: bool,
    pub(crate) internal: bool,
    pub(crate) nowait: bool,
}

impl Default for AMQPChannelOptions {
    fn default() -> Self {
        Self {
            exchange: ExchangeKind::Custom(String::new()),
            routing_key: String::new(),
            passive: false,
            durable: false,
            exclusive: false,
            auto_delete: false,
            internal: false,
            nowait: false,
        }
    }
}

impl AMQPChannelOptions {
    pub fn with_exchange(mut self, value: ExchangeKind) -> Self {
        self.exchange = value;
        self
    }

    pub fn with_routing_key(mut self, value: impl ToString) -> Self {
        self.routing_key = value.to_string();
        self
    }

    pub fn passive(mut self) -> Self {
        self.passive = true;
        self
    }

    pub fn durable(mut self) -> Self {
        self.durable = true;
        self
    }

    pub fn exclusive(mut self) -> Self {
        self.exclusive = true;
        self
    }
    pub fn auto_delete(mut self) -> Self {
        self.auto_delete = true;
        self
    }

    pub fn internal(mut self) -> Self {
        self.internal = true;
        self
    }

    pub fn nowait(mut self) -> Self {
        self.nowait = true;
        self
    }
}
