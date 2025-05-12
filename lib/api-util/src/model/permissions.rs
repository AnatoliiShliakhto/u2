#![allow(dead_code)]
use super::capabilities::Capabilities;
use ::base64::{Engine, engine::general_purpose::STANDARD};
use ::bitflags::Flags;

#[derive(Clone)]
pub struct Permissions {
    inner: Vec<Capabilities>,
}

impl Permissions {
    pub fn new(permissions: &[Capabilities]) -> Self {
        Self {
            inner: permissions.to_vec(),
        }
    }

    pub fn get(&self, index: u16) -> Capabilities {
        if index >= self.inner.len() as u16 {
            return Capabilities::NONE;
        }
        self.inner[index as usize].clone()
    }

    pub fn as_vec(&self) -> Vec<Capabilities> {
        self.inner.clone()
    }

    pub fn as_slice(&self) -> &[Capabilities] {
        self.inner.as_slice()
    }

    pub fn init(ids: &[u16], permissions: Vec<(u16, u8)>) -> Self {
        let mut inner = <Vec<Capabilities>>::with_capacity(ids.len());

        for _ in 0..ids.len() {
            inner.push(Capabilities::NONE);
        }

        for i in 0..ids.len() {
            permissions
                .iter()
                .filter(|(id, _)| id == &ids[i])
                .for_each(|(_, bits)| {
                    inner[i] = inner[i]
                        .clone()
                        .union(Capabilities::from_bits_truncate(*bits));
                });
        }

        Self { inner }
    }

    pub fn encode_to_base64(&self) -> String {
        STANDARD.encode(self.inner.iter().map(Flags::bits).collect::<Vec<u8>>())
    }

    pub fn decode_from_base64(encoded: impl ToString) -> Self {
        let inner = STANDARD
            .decode(encoded.to_string())
            .unwrap_or_default()
            .into_iter()
            .map(Capabilities::from_bits_truncate)
            .collect();

        Self { inner }
    }
}
