use crate::de::CellDeserializer;
use crate::error::SerdeTonError;
use serde::de::Visitor;
use serde::forward_to_deserialize_any;
use tonlib_core::cell::dict::predefined_readers::val_reader_snake_formatted_string;

impl<'de> serde::de::Deserializer<'de> for &mut CellDeserializer<'de> {
    type Error = SerdeTonError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    forward_to_deserialize_any! {
        i128 u128 f32 f64 char
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(
            self.parser
                .load_bit()
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // For some reason `self.parser.load_i8` causes panic
        visitor.visit_i8(
            self.parser
                .load_u8(8)
                .map_err(|err| Self::Error::FieldError(err.to_string()))? as i8,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(
            self.parser
                .load_i16(16)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(
            self.parser
                .load_i32(32)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(
            self.parser
                .load_i64(64)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(
            self.parser
                .load_u8(8)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(
            self.parser
                .load_u16(16)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(
            self.parser
                .load_u32(32)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(
            self.parser
                .load_u64(64)
                .map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let string = val_reader_snake_formatted_string(&mut self.parser)
            .map_err(|err| Self::Error::FieldError(err.to_string()))?;

        visitor.visit_string(
            String::from_utf8(string).map_err(|err| Self::Error::FieldError(err.to_string()))?,
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }
}
