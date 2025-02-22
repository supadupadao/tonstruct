use crate::from_cell::FromCell;
use crate::to_cell::ToCell;
use crate::utils::CastErrorToAnyhow;
use num_bigint::BigInt;
use num_bigint::Sign::Minus;
use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};
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

macro_rules! try_into {
    ($structname: ty) => {
        impl<const SIZE: usize> TryFrom<Int<SIZE>> for $structname {
            type Error = anyhow::Error;

            fn try_from(value: Int<SIZE>) -> Result<Self, Self::Error> {
                value
                    .0
                    .try_into()
                    .map_err(|err| anyhow::Error::msg(format!("Cannot cast Int: {:?}", err)))
            }
        }
    };
}
try_into!(u32);
try_into!(u64);
try_into!(u128);
try_into!(usize);
try_into!(i32);
try_into!(i64);
try_into!(i128);
try_into!(isize);

impl<const SIZE: usize> Display for Int<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const SIZE: usize> LowerHex for Int<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl<const SIZE: usize> UpperHex for Int<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.0)
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
    fn test_try_into() {
        assert_eq!(
            <Int<BITS> as TryInto<u32>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as u32
        );
        assert_eq!(
            <Int<BITS> as TryInto<u64>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as u64
        );
        assert_eq!(
            <Int<BITS> as TryInto<u128>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as u128
        );
        assert_eq!(
            <Int<BITS> as TryInto<usize>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE
        );
        assert_eq!(
            <Int<BITS> as TryInto<i32>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as i32
        );
        assert_eq!(
            <Int<BITS> as TryInto<i64>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as i64
        );
        assert_eq!(
            <Int<BITS> as TryInto<i128>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as i128
        );
        assert_eq!(
            <Int<BITS> as TryInto<isize>>::try_into(Int(BigInt::from(INT_VALUE))).unwrap(),
            INT_VALUE as isize
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

    #[test]
    fn test_fmt() {
        let value = Int::<32>::from(BigInt::from(0x1234abcdi32));
        assert_eq!("Int(305441741)", format!("{:?}", value));
        assert_eq!("305441741", format!("{}", value));
        assert_eq!("0x1234abcd", format!("{:x}", value));
        assert_eq!("0x1234ABCD", format!("{:X}", value));
    }
}
