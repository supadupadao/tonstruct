use crate::common::TonAddress;
use crate::Result;

pub trait TonstructParser {
    fn load_bits(&mut self, bits: usize) -> Result<Vec<u8>>;

    fn load_address(&mut self) -> Result<TonAddress>;
}

pub trait FromCell: Sized {
    fn load(source: &mut impl TonstructParser) -> Result<Self>;
}
