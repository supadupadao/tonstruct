use crate::{FromCell, ToCell, TonstructBuilder, TonstructParser};
use num_bigint::BigInt;

#[derive(Debug, PartialEq, Default)]
pub struct Int<const SIZE: usize>(BigInt);

impl<const SIZE: usize> FromCell for Int<SIZE> {
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        source.load_int(SIZE).map(Self)
    }
}

impl<const SIZE: usize> ToCell for Int<SIZE> {
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_int(&self.0, SIZE)?;
        Ok(())
    }
}
