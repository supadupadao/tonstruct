use crate::error::SerdeTonError;
use crate::CellSerializer;
use serde::Serialize;

impl serde::ser::SerializeStructVariant for &mut CellSerializer {
    type Ok = ();
    type Error = SerdeTonError;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
