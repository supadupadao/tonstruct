use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use num_bigint::BigUint;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct Coins(BigUint);

impl From<BigUint> for Coins {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

impl From<Coins> for BigUint {
    fn from(coins: Coins) -> Self {
        coins.0
    }
}

macro_rules! try_into {
    ($structname: ty) => {
        impl TryFrom<Coins> for $structname {
            type Error = anyhow::Error;

            fn try_from(value: Coins) -> Result<Self, Self::Error> {
                value
                    .0
                    .try_into()
                    .map_err(|err| anyhow::Error::msg(format!("Cannot cast Coins: {:?}", err)))
            }
        }
    };
}
try_into!(u32);
try_into!(u64);
try_into!(u128);
try_into!(usize);

impl ToCell for Coins {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_coins(&self.0).map_err_to_anyhow()
    }
}

impl FromCell for Coins {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_coins().map_err_to_anyhow().map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COINS_VALUE: usize = 100500;

    #[test]
    fn test_from() {
        assert_eq!(
            Coins::from(BigUint::from(COINS_VALUE)),
            Coins(BigUint::from(COINS_VALUE))
        );
    }

    #[test]
    fn test_into() {
        assert_eq!(
            <Coins as Into<BigUint>>::into(Coins(BigUint::from(COINS_VALUE))),
            BigUint::from(COINS_VALUE)
        );
    }

    #[test]
    fn test_try_into() {
        assert_eq!(
            <Coins as TryInto<u32>>::try_into(Coins(BigUint::from(COINS_VALUE))).unwrap(),
            COINS_VALUE as u32
        );
        assert_eq!(
            <Coins as TryInto<u64>>::try_into(Coins(BigUint::from(COINS_VALUE))).unwrap(),
            COINS_VALUE as u64
        );
        assert_eq!(
            <Coins as TryInto<u128>>::try_into(Coins(BigUint::from(COINS_VALUE))).unwrap(),
            COINS_VALUE as u128
        );
        assert_eq!(
            <Coins as TryInto<usize>>::try_into(Coins(BigUint::from(COINS_VALUE))).unwrap(),
            COINS_VALUE
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Coins::default(), Coins(BigUint::ZERO));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_coins(&BigUint::from(COINS_VALUE))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Coins::from_cell(cell).unwrap(),
            Coins(BigUint::from(COINS_VALUE))
        );
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            Coins(BigUint::from(COINS_VALUE)).to_cell().unwrap(),
            CellBuilder::new()
                .store_coins(&BigUint::from(COINS_VALUE))
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = Coins::from(BigUint::from(COINS_VALUE));
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Coins::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }
}
