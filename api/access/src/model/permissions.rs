#![allow(dead_code)]
use super::capabilities::Capabilities;
use ::base64::{Engine, engine::general_purpose::STANDARD};
use ::bitflags::Flags;
use ::std::collections::HashMap;

#[derive(Clone)]
pub struct Permissions {
    inner: Vec<Capabilities>,
}

impl Permissions {
    /// Creates a new Permissions instance from a slice of capabilities
    pub fn new(permissions: &[Capabilities]) -> Self {
        Self {
            inner: permissions.to_vec(),
        }
    }

    /// Gets capability at the specified index
    pub fn get(&self, index: u16) -> Option<Capabilities> {
        self.inner.get(index as usize).copied()
    }

    /// Gets capability at the specified index, returns Capabilities::NONE if out of bounds
    pub fn get_or_default(&self, index: u16) -> Capabilities {
        self.inner
            .get(index as usize)
            .copied()
            .unwrap_or(Capabilities::NONE)
    }

    pub fn as_slice(&self) -> &[Capabilities] {
        &self.inner
    }

    pub fn as_vec(&self) -> &Vec<Capabilities> {
        &self.inner
    }

    /// Initializes permissions from IDs and permission mappings
    pub fn init(permissions_map: &[u16], permissions: HashMap<u16, u8>) -> Self {
        let inner: Vec<Capabilities> = permissions_map
            .iter()
            .map(|id| {
                permissions
                    .get(id)
                    .copied()
                    .map(Capabilities::from_bits_truncate)
                    .unwrap_or(Capabilities::NONE)
            })
            .collect();

        Self { inner }
    }

    /// Encodes permissions to base64 string
    pub fn encode_to_base64(&self) -> String {
        let bytes: Vec<u8> = self.inner.iter().map(Flags::bits).collect();
        STANDARD.encode(bytes)
    }

    /// Decodes permissions from base64 string
    pub fn decode_from_base64(
        encoded: impl AsRef<str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = STANDARD.decode(encoded.as_ref())?;

        // Validate that all bytes represent valid capabilities
        let inner: Result<Vec<Capabilities>, _> = bytes
            .into_iter()
            .map(|byte| {
                Capabilities::from_bits(byte)
                    .ok_or_else(|| format!("Invalid capability bits: {:#04x}", byte))
            })
            .collect();

        match inner {
            Ok(capabilities) => Ok(Self {
                inner: capabilities,
            }),
            Err(e) => Err(e.into()),
        }
    }

    /// Decodes permissions from base64 string, returns empty on error
    pub fn decode_from_base64_or_empty(encoded: impl AsRef<str>) -> Self {
        Self::decode_from_base64(encoded).unwrap_or_else(|_| Self { inner: Vec::new() })
    }

    /// Returns the number of permissions
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if permissions are empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
