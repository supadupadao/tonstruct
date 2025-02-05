use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tonlib_core::TonAddress;

#[derive(PartialEq, Debug)]
pub struct Address(TonAddress);

impl Address {
    pub fn new(address: TonAddress) -> Self {
        Address(address)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hash = self.0.hash_part.as_slice();
        let mut tup = serializer.serialize_tuple(4 + hash.len())?;
        tup.serialize_element(&true)?;
        tup.serialize_element(&false)?;
        tup.serialize_element(&false)?;
        let workchain = self.0.workchain as u8 & 0xFF;
        tup.serialize_element(&workchain)?;
        for byte in hash {
            tup.serialize_element(byte)?;
        }
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
