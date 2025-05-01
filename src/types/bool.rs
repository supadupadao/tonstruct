use crate::from_cell::TonstructParser;
use crate::{FromCell, ToCell, TonstructBuilder};

impl ToCell for bool {
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_bit(true)?;
        Ok(())
    }
}

impl FromCell for bool {
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        source.load_bit()
    }
}
