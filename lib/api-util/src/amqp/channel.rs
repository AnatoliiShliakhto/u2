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

    fn set_flag(mut self, flag: fn(&mut Self)) -> Self {
        flag(&mut self);
        self
    }

    pub fn with_passive(self) -> Self {
        self.set_flag(|opts| opts.passive = true)
    }

    pub fn with_durable(self) -> Self {
        self.set_flag(|opts| opts.durable = true)
    }

    pub fn with_exclusive(self) -> Self {
        self.set_flag(|opts| opts.exclusive = true)
    }

    pub fn with_auto_delete(self) -> Self {
        self.set_flag(|opts| opts.auto_delete = true)
    }

    pub fn with_internal(self) -> Self {
        self.set_flag(|opts| opts.internal = true)
    }

    pub fn with_nowait(self) -> Self {
        self.set_flag(|opts| opts.nowait = true)
    }
}
