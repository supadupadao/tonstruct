use crate::error::{Error, Result};
use crate::fields::Address;
use crate::from_cell::TonstructParser;
use crate::to_cell::TonstructBuilder;
use crate::{FromCell, ToCell, TonAddress};
use num_bigint::{BigInt, BigUint};
use tonlib_core::cell::{Cell, CellBuilder, CellParser};

impl From<tonlib_core::TonAddress> for TonAddress {
    fn from(value: tonlib_core::TonAddress) -> Self {
        Self {
            workchain: value.workchain,
            address: value.hash_part.to_vec(),
        }
    }
}

impl TryFrom<&TonAddress> for tonlib_core::TonAddress {
    type Error = Error;

    fn try_from(value: &TonAddress) -> Result<Self> {
        Ok(Self {
            workchain: value.workchain,
            hash_part: value
                .address
                .clone()
                .try_into()
                .map_err(|err| Error::TypeError(format!("{:?}", err)))?,
        })
    }
}

impl From<tonlib_core::TonAddress> for Address {
    fn from(value: tonlib_core::TonAddress) -> Self {
        Self::from(<tonlib_core::TonAddress as Into<TonAddress>>::into(value))
    }
}

impl TonstructParser for CellParser<'_> {
    fn load_bit(&mut self) -> Result<bool> {
        self.load_bit().map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_bits(&mut self, bits: usize) -> Result<Vec<u8>> {
        self.load_bits(bits)
            .map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_address(&mut self) -> Result<TonAddress> {
        self.load_address()
            .map(Into::into)
            .map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_coins(&mut self) -> Result<BigUint> {
        self.load_coins()
            .map(Into::into)
            .map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_int(&mut self, bits: usize) -> Result<BigInt> {
        self.load_int(bits)
            .map(Into::into)
            .map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_uint(&mut self, bits: usize) -> Result<BigUint> {
        self.load_uint(bits)
            .map(Into::into)
            .map_err(|e| Error::ReadError(e.to_string()))
    }

    fn load_string(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        loop {
            let remaining_bits = self.remaining_bits();
            if remaining_bits < 8 {
                // buf.push(parser.load_u8(remaining_bits)?);
                // For some reason adds \0 char to end
                break;
            }

            if remaining_bits == 0 {
                break;
            }

            buf.push(
                self.load_u8(8)
                    .map_err(|e| Error::ReadError(e.to_string()))?,
            );
        }

        String::from_utf8(buf).map_err(|e| Error::ReadError(e.to_string()))
    }
}

impl TonstructBuilder for CellBuilder {
    fn store_address(&mut self, address: &TonAddress) -> Result<&mut Self> {
        self.store_address(&address.try_into()?)
            .map_err(|e| Error::WriteError(e.to_string()))
    }

    fn store_bit(&mut self, bit: bool) -> Result<&mut Self> {
        self.store_bit(bit)
            .map_err(|e| Error::WriteError(e.to_string()))
    }

    fn store_coins(&mut self, coins: &BigUint) -> Result<&mut Self> {
        self.store_coins(coins)
            .map_err(|e| Error::WriteError(e.to_string()))
    }

    fn store_int(&mut self, int: &BigInt, bits: usize) -> Result<&mut Self> {
        self.store_int(bits, int)
            .map_err(|e| Error::WriteError(e.to_string()))
    }

    fn store_uint(&mut self, uint: &BigUint, bits: usize) -> Result<&mut Self> {
        self.store_uint(bits, uint)
            .map_err(|e| Error::WriteError(e.to_string()))
    }

    fn store_string(&mut self, string: &str) -> Result<&mut Self> {
        self.store_string(string)
            .map_err(|e| Error::WriteError(e.to_string()))
    }
}

pub fn from_tonlib_core_cell<T: FromCell>(cell: Cell) -> Result<T> {
    let mut parser = cell.parser();

    T::load(&mut parser)
}

pub fn to_tonlib_core_cell<T: ToCell>(object: T) -> Result<Cell> {
    let mut buf = CellBuilder::new();
    ToCell::store(&object, &mut buf)?;
    buf.build().map_err(|e| Error::WriteError(e.to_string()))
}
