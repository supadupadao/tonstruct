use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use num_bigint::BigUint;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct Uint<const SIZE: usize>(BigUint);

impl<const SIZE: usize> From<BigUint> for Uint<SIZE> {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

impl<const SIZE: usize> From<Uint<SIZE>> for BigUint {
    fn from(value: Uint<SIZE>) -> Self {
        value.0
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

#[cfg(test)]
mod tests {
    use super::*;

    const INT_VALUE: usize = 100500;
    const BITS: usize = 69;

    #[test]
    fn test_from() {
        assert_eq!(
            Uint::<BITS>::from(BigUint::from(INT_VALUE)),
            Uint(BigUint::from(INT_VALUE))
        );
    }

    #[test]
    fn test_into() {
        assert_eq!(
            <Uint::<BITS> as Into<BigUint>>::into(Uint(BigUint::from(INT_VALUE))),
            BigUint::from(INT_VALUE)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Uint::<BITS>::default(), Uint(BigUint::ZERO));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_uint(BITS, &BigUint::from(INT_VALUE))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Uint::<BITS>::from_cell(cell).unwrap(),
            Uint(BigUint::from(INT_VALUE))
        );
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            Uint::<BITS>(BigUint::from(INT_VALUE)).to_cell().unwrap(),
            CellBuilder::new()
                .store_uint(BITS, &BigUint::from(INT_VALUE))
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = Uint::<BITS>::from(BigUint::from(INT_VALUE));
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Uint::<BITS>::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
