use crate::{common::TonAddress, Result};

pub trait TonstructBuilder {
    fn store_address(&mut self, address: TonAddress) -> Result<&mut Self>;
}

pub trait ToCell<T: TonstructBuilder> {
    fn store<'a>(&self, builder: &'a mut T) -> Result<&'a mut T>;
}
