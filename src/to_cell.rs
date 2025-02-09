use crate::utils::CastErrorToAnyhow;

pub trait ToCell {
    fn to_cell(&self) -> anyhow::Result<tonlib_core::cell::Cell> {
        let mut buf = tonlib_core::cell::CellBuilder::new();
        self.store(&mut buf)?;
        buf.build().map_err_to_anyhow()
    }

    fn store<'a>(
        &self,
        builder: &'a mut tonlib_core::cell::CellBuilder,
    ) -> anyhow::Result<&'a mut tonlib_core::cell::CellBuilder>;
}
