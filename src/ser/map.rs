use crate::error::SerdeTonError;
use crate::CellSerializer;
use serde::Serialize;

impl serde::ser::SerializeMap for &mut CellSerializer {
    type Ok = ();
    type Error = SerdeTonError;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
