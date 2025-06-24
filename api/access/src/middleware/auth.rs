use crate::model::Permissions;
use ::serde::{Deserialize, Serializer};

#[derive(Clone)]
pub struct Auth {
    pub id: String,
    pub permissions: Permissions,
}

impl Auth {
    pub fn serialize<S: Serializer>(auth: &Option<Auth>, serializer: S) -> Result<S::Ok, S::Error> {
        match auth {
            Some(auth) => serializer
                .serialize_str(&[auth.id.clone(), auth.permissions.encode_to_base64()].concat()),
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

        let permissions =
            Permissions::decode_from_base64_or_empty(id.split_off(20));

        Ok(Some(Self { id, permissions }))
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            id: String::new(),
            permissions: Permissions::new(&[]),
        }
    }
}