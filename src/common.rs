#[derive(Debug, PartialEq, Eq)]
pub struct TonAddress {
    pub workchain: i32,
    pub address: Vec<u8>,
}

impl TonAddress {
    pub fn null() -> Self {
        Self {
            workchain: 0,
            address: [0; 32].to_vec(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Cell(Vec<u8>);
