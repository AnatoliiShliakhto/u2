use super::{BasicProperties, FieldTable};

#[derive(Default)]
pub struct AMQPMessageOptions {
    pub(crate) mandatory: bool,
    pub(crate) immediate: bool,
    pub(crate) properties: BasicProperties,
}

impl AMQPMessageOptions {
    pub fn mandatory(mut self) -> Self {
        self.mandatory = true;
        self
    }

    pub fn immediate(mut self) -> Self {
        self.immediate = true;
        self
    }

    fn with_string_property<F>(mut self, value: impl ToString, setter: F) -> Self
    where
        F: FnOnce(BasicProperties, String) -> BasicProperties,
    {
        self.properties = setter(self.properties, value.to_string());
        self
    }

    fn with_property<T, F>(mut self, value: T, setter: F) -> Self
    where
        F: FnOnce(BasicProperties, T) -> BasicProperties,
    {
        self.properties = setter(self.properties, value);
        self
    }

    pub fn with_app_id(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_app_id(val.into()))
    }

    pub fn with_content_type(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_content_type(val.into()))
    }

    pub fn with_headers(self, value: FieldTable) -> Self {
        self.with_property(value, |props, val| props.with_headers(val))
    }

    pub fn with_delivery_mode(self, value: u8) -> Self {
        self.with_property(value, |props, val| props.with_delivery_mode(val))
    }

    pub fn with_priority(self, value: u8) -> Self {
        self.with_property(value, |props, val| props.with_priority(val))
    }

    pub fn with_correlation_id(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_correlation_id(val.into()))
    }

    pub fn with_reply_to(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_reply_to(val.into()))
    }

    pub fn with_expiration(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_expiration(val.into()))
    }

    pub fn with_message_id(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_message_id(val.into()))
    }

    pub fn with_timestamp(self, value: u64) -> Self {
        self.with_property(value, |props, val| props.with_timestamp(val))
    }

    pub fn with_type(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_type(val.into()))
    }

    pub fn with_user_id(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_user_id(val.into()))
    }

    pub fn with_cluster_id(self, value: impl ToString) -> Self {
        self.with_string_property(value, |props, val| props.with_cluster_id(val.into()))
    }
}
