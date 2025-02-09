use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use tonlib_core::cell::{CellBuilder, CellParser};
use tonlib_core::TonAddress;

#[derive(Debug, PartialEq)]
pub struct Address(TonAddress);

impl From<TonAddress> for Address {
    fn from(value: TonAddress) -> Self {
        Self(value)
    }
}

impl FromCell for Address {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.load_address().map(Self).map_err_to_anyhow()
    }
}

impl ToCell for Address {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_address(&self.0).map_err_to_anyhow()
    }
}
