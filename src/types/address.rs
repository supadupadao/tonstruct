use crate::from_cell::TonstructParser;
use crate::to_cell::TonstructBuilder;
use crate::{FromCell, Result, ToCell, TonAddress};

#[derive(Debug, PartialEq)]
pub struct Address(TonAddress);

impl From<TonAddress> for Address {
    fn from(value: TonAddress) -> Self {
        Self(value)
    }
}

impl From<Address> for TonAddress {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl Default for Address {
    fn default() -> Self {
        Self(TonAddress::null())
    }
}

impl FromCell for Address {
    fn load(parser: &mut impl TonstructParser) -> Result<Self> {
        parser.load_address().map(Self)
    }
}

impl ToCell for Address {
    fn store(&self, builder: &mut impl TonstructBuilder) -> Result<()> {
        builder.store_address(&self.0)?;
        Ok(())
    }
}
