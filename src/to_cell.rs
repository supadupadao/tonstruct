use crate::{common::TonAddress, Result};
use num_bigint::{BigInt, BigUint};

pub trait TonstructBuilder {
    fn store_address(&mut self, address: &TonAddress) -> Result<&mut Self>;

    fn store_bit(&mut self, bit: bool) -> Result<&mut Self>;
    fn store_coins(&mut self, coins: &BigUint) -> Result<&mut Self>;
    fn store_int(&mut self, int: &BigInt, bits: usize) -> Result<&mut Self>;
    fn store_uint(&mut self, uint: &BigUint, bits: usize) -> Result<&mut Self>;
    fn store_string(&mut self, string: &str) -> Result<&mut Self>;
}

pub trait ToCell {
    fn store(&self, builder: &mut impl TonstructBuilder) -> Result<()>;
}
