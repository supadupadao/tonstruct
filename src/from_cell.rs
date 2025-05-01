use crate::common::TonAddress;
use crate::Result;
use num_bigint::{BigInt, BigUint};

pub trait TonstructParser {
    fn load_bit(&mut self) -> Result<bool>;
    fn load_bits(&mut self, bits: usize) -> Result<Vec<u8>>;

    fn load_address(&mut self) -> Result<TonAddress>;
    fn load_coins(&mut self) -> Result<BigUint>;
    fn load_int(&mut self, bits: usize) -> Result<BigInt>;
    fn load_uint(&mut self, bits: usize) -> Result<BigUint>;
    fn load_string(&mut self) -> Result<String>;
}

pub trait FromCell: Sized {
    fn load(source: &mut impl TonstructParser) -> Result<Self>;
}
