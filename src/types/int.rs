use crate::from_cell::FromCell;
use crate::to_cell::ToCell;
use crate::utils::CastErrorToAnyhow;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;
use std::fmt::{Debug, Formatter};
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq)]
pub struct Int<const SIZE: usize>(BigInt);

impl<const SIZE: usize> From<BigInt> for Int<SIZE> {
    fn from(value: BigInt) -> Self {
        Self(value)
    }
}

impl<const SIZE: usize> ToCell for Int<SIZE> {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        let (sign, value) = self.0.clone().into_parts();
        builder.store_bit(sign == Plus)?;
        builder.store_uint(SIZE - 1, &value)?;
        Ok(builder)
    }
}

impl<const SIZE: usize> FromCell for Int<SIZE> {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_int(SIZE).map(Self).map_err_to_anyhow()
    }
}
