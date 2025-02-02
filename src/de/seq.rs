use crate::error::SerdeTonError;
use crate::CellDeserializer;
use serde::de::DeserializeSeed;

impl<'de> serde::de::SeqAccess<'de> for &mut CellDeserializer<'de> {
    type Error = SerdeTonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        println!("HERE {:?}", self.parser.remaining_bits());
        seed.deserialize(&mut **self).map(Some)
    }
}
