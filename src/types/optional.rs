use crate::{FromCell, ToCell, TonstructBuilder, TonstructParser};

impl<T> FromCell for Option<T>
where
    T: FromCell,
{
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        match source.load_bit()? {
            true => T::load(source).map(Some),
            false => Ok(None),
        }
    }
}

impl<T> ToCell for Option<T>
where
    T: ToCell,
{
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_bit(self.is_some())?;
        if let Some(inner) = self {
            inner.store(builder)?;
        }
        Ok(())
    }
}
