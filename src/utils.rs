use tonlib_core::cell::TonCellError;

pub trait CastErrorToAnyhow<R> {
    fn map_err_to_anyhow(self) -> anyhow::Result<R>;
}

impl<R> CastErrorToAnyhow<R> for Result<R, TonCellError> {
    fn map_err_to_anyhow(self) -> anyhow::Result<R> {
        self.map_err(anyhow::Error::msg)
    }
}
