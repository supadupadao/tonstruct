use crate::{FromCell, ToCell, TonstructBuilder, TonstructParser};
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Default)]
pub struct Coins(BigUint);

impl FromCell for Coins {
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        source.load_coins().map(Self)
    }
}

impl ToCell for Coins {
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_coins(&self.0)?;
        Ok(())
    }
}
