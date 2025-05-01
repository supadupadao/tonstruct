use crate::{FromCell, ToCell, TonstructBuilder, TonstructParser};

impl FromCell for String {
    fn load(source: &mut impl TonstructParser) -> crate::Result<Self> {
        source.load_string()
    }
}

impl ToCell for String {
    fn store(&self, builder: &mut impl TonstructBuilder) -> crate::Result<()> {
        builder.store_string(self)?;
        Ok(())
    }
}
