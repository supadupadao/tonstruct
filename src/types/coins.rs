use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use num_bigint::BigUint;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq)]
pub struct Coins(BigUint);

impl From<BigUint> for Coins {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

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
