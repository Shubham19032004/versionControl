use std::{borrow::Borrow, fmt::Display, ops::Deref};

use anyhow::anyhow;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
pub struct Hash {
    hash: [u8; 20],
}

impl Hash {
    pub fn new(hash: [u8; 20]) -> Self {
        Self { hash }
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.hash
    }
}

impl Borrow<[u8; 20]> for Hash {
    fn borrow(&self) -> &[u8; 20] {
        self.hash.borrow()
    }
}

impl Deref for Hash {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.hash
    }
}

impl IntoIterator for &Hash {
    type Item = u8;

    type IntoIter = std::array::IntoIter<u8, 20>;

    fn into_iter(self) -> Self::IntoIter {
        self.hash.into_iter()
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let value = if value.len() == 40 {
            hex::decode(value).map_err(|_error| {
                anyhow!("Error decoding value in preparation for converting to Hash")
            })?
        } else {
            value
        };

        Ok(Self {
            hash: value
                .try_into()
                .map_err(|_error| anyhow!("Error attempting to convert Vec<u8> to [u8; 20]"))?,
        })
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.hash))
    }
}
