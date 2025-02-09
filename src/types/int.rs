use crate::from_cell::FromCell;
use crate::to_cell::ToCell;
use crate::utils::CastErrorToAnyhow;
use num_bigint::BigInt;
use num_bigint::Sign::Minus;
use std::fmt::Debug;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct Int<const SIZE: usize>(BigInt);

impl<const SIZE: usize> From<BigInt> for Int<SIZE> {
    fn from(value: BigInt) -> Self {
        Self(value)
    }
}

impl<const SIZE: usize> From<Int<SIZE>> for BigInt {
    fn from(value: Int<SIZE>) -> Self {
        value.0
    }
}

impl<const SIZE: usize> ToCell for Int<SIZE> {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        let (sign, value) = self.0.clone().into_parts();
        builder.store_bit(sign == Minus)?;
        builder.store_uint(SIZE - 1, &value)?;
        Ok(builder)
    }
}

impl<const SIZE: usize> FromCell for Int<SIZE> {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_int(SIZE).map(Self).map_err_to_anyhow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    const INT_VALUE: usize = 100500;
    const BITS: usize = 69;

    #[test]
    fn test_from() {
        assert_eq!(
            Int::<BITS>::from(BigInt::from(INT_VALUE)),
            Int(BigInt::from(INT_VALUE))
        );
    }

    #[test]
    fn test_into() {
        assert_eq!(
            <Int::<BITS> as Into<BigInt>>::into(Int(BigInt::from(INT_VALUE))),
            BigInt::from(INT_VALUE)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Int::<BITS>::default(), Int(BigInt::ZERO));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_bit(false)
            .unwrap()
            .store_uint(BITS - 1, &BigUint::from(INT_VALUE))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Int::<BITS>::from_cell(cell).unwrap(),
            Int(BigInt::from(INT_VALUE))
        );
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            Int::<BITS>(BigInt::from(INT_VALUE)).to_cell().unwrap(),
            CellBuilder::new()
                .store_bit(false)
                .unwrap()
                .store_uint(BITS - 1, &BigUint::from(INT_VALUE))
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = Int::<BITS>::from(BigInt::from(INT_VALUE));
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Int::<BITS>::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
