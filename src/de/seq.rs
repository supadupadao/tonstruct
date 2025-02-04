use crate::error::SerdeTonError;
use crate::CellDeserializer;
use serde::de::{DeserializeSeed, SeqAccess};

pub struct Seq<'a, 'de> {
    pub de: &'a mut CellDeserializer<'de>,
    pub len: usize,
}

impl<'a, 'de> SeqAccess<'de> for Seq<'a, 'de> {
    type Error = SerdeTonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;
        let value = DeserializeSeed::deserialize(seed, &mut *self.de)?;
        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
