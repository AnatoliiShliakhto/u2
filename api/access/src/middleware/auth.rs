use crate::model::Permissions;
use ::serde::{Deserialize, Serializer};
use ::std::borrow::Cow;

#[derive(Clone)]
pub struct Auth<'a> {
    pub id: Cow<'a, str>,
    pub permissions: Permissions,
}

impl Auth<'_> {
    pub fn serialize<S: Serializer>(auth: &Option<Auth>, serializer: S) -> Result<S::Ok, S::Error> {
        match auth {
            Some(auth) => serializer
                .serialize_str(&[&auth.id, &*auth.permissions.encode_to_base64()].concat()),
            _ => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Self>, D::Error> {
        let mut id = String::deserialize(deserializer)?;

        if id.len() < 20 {
            return Ok(None);
        }

        let permissions = Permissions::decode_from_base64_or_empty(id.split_off(20));

        Ok(Some(Self {
            id: Cow::Owned(id),
            permissions,
        }))
    }
}

impl Default for Auth<'_> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            permissions: Permissions::new(&[]),
        }
    }
}
