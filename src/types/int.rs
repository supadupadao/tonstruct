use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct Int<const BITS: usize>([bool; BITS]);

impl<const BITS: usize> Int<BITS> {
    pub fn from_usize(value: usize) -> Int<BITS> {
        let mut value = value;

        let mut result = [false; BITS];
        for bit in 0..BITS {
            result[bit] = value & 1 == 1;
            value = value >> 1;
        }
        Self(result)
    }
}

impl<const BITS: usize> Serialize for Int<BITS> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(BITS)?;
        for bit in self.0.iter() {
            tup.serialize_element(bit)?;
        }
        tup.end()
    }
}

impl<'de, const BITS: usize> Deserialize<'de> for Int<BITS> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoolVisitor<const BITS: usize>;

        impl<'de, const BITS: usize> Visitor<'de> for BoolVisitor<BITS> {
            type Value = Int<BITS>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("BitArray")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = 0;
                while let Some(a) = seq.next_element::<bool>()? {
                    result <<= 1;
                    result |= a as usize;
                }
                Ok(Int::from_usize(result))
            }
        }

        deserializer.deserialize_tuple(BITS, BoolVisitor)
    }
}
