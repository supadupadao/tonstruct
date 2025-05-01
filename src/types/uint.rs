use crate::{FromCell, ToCell, TonstructBuilder, TonstructParser};
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Default)]
pub struct Uint<const SIZE: usize>(BigUint);

impl<const SIZE: usize> FromCell for Uint<SIZE> {
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        source.load_uint(SIZE).map(Self)
    }
}

impl<const SIZE: usize> ToCell for Uint<SIZE> {
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_uint(&self.0, SIZE)?;
        Ok(())
    }
}
