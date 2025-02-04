use crate::error::SerdeTonError;
use crate::CellSerializer;
use serde::Serialize;

impl serde::ser::SerializeTuple for &mut CellSerializer {
    type Ok = ();
    type Error = SerdeTonError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
