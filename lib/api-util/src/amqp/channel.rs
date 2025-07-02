use super::ExchangeKind;
use ::std::borrow::Cow;

#[derive(Default)]
pub struct AMQPChannelOptions<'a> {
    pub(crate) exchange: ExchangeKind,
    pub(crate) routing_key: Cow<'a, str>,
    pub(crate) passive: bool,
    pub(crate) durable: bool,
    pub(crate) exclusive: bool,
    pub(crate) auto_delete: bool,
    pub(crate) internal: bool,
    pub(crate) nowait: bool,
}

impl<'a> AMQPChannelOptions<'a> {
    pub fn with_exchange(mut self, value: ExchangeKind) -> Self {
        self.exchange = value;
        self
    }

    pub fn with_routing_key(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.routing_key = value.into();
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
