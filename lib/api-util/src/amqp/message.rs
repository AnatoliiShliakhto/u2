use super::{BasicProperties, FieldTable};

#[derive(Default)]
pub struct AMQPMessageOptions {
    pub(crate) app_id: bool,
    pub(crate) mandatory: bool,
    pub(crate) immediate: bool,
    pub(crate) properties: BasicProperties,
}

impl AMQPMessageOptions {
    pub fn with_app_id(mut self) -> Self {
        self.app_id = true;
        self
    }

    pub fn with_mandatory(mut self) -> Self {
        self.mandatory = true;
        self
    }

    pub fn with_immediate(mut self) -> Self {
        self.immediate = true;
        self
    }

    pub fn with_content_type(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_content_type(value.to_string().into());
        self
    }

    pub fn with_headers(mut self, value: FieldTable) -> Self {
        self.properties = self.properties.with_headers(value);
        self
    }

    pub fn with_delivery_mode(mut self, value: u8) -> Self {
        self.properties = self.properties.with_delivery_mode(value);
        self
    }

    pub fn with_priority(mut self, value: u8) -> Self {
        self.properties = self.properties.with_priority(value);
        self
    }

    pub fn with_correlation_id(mut self, value: impl ToString) -> Self {
        self.properties = self
            .properties
            .with_correlation_id(value.to_string().into());
        self
    }

    pub fn with_reply_to(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_reply_to(value.to_string().into());
        self
    }

    pub fn with_expiration(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_expiration(value.to_string().into());
        self
    }

    pub fn with_message_id(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_message_id(value.to_string().into());
        self
    }

    pub fn with_timestamp(mut self, value: u64) -> Self {
        self.properties = self.properties.with_timestamp(value);
        self
    }

    pub fn with_type(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_type(value.to_string().into());
        self
    }

    pub fn with_user_id(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_user_id(value.to_string().into());
        self
    }

    pub fn with_cluster_id(mut self, value: impl ToString) -> Self {
        self.properties = self.properties.with_cluster_id(value.to_string().into());
        self
    }
}
