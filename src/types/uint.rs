use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use num_bigint::BigUint;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq)]
pub struct Uint<const SIZE: usize>(BigUint);

impl<const SIZE: usize> From<BigUint> for Uint<SIZE> {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

impl<const SIZE: usize> ToCell for Uint<SIZE> {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_uint(SIZE, &self.0).map_err_to_anyhow()
    }
}

impl<const SIZE: usize> FromCell for Uint<SIZE> {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_uint(SIZE).map(Self).map_err_to_anyhow()
    }
}
