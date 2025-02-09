use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};

impl<T> ToCell for Option<T>
where
    T: ToCell,
{
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_bit(self.is_some())?;
        if let Some(inner) = self {
            inner.store(builder)?;
        }
        Ok(builder)
    }
}

impl<T> FromCell for Option<T>
where
    T: FromCell,
{
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        match parser.load_bit()? {
            true => T::load(parser).map(Some),
            false => Ok(None),
        }
    }
}
