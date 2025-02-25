//! In this example we will implement custom type with our own serialization / deserialization logic

use tonlib_core::cell::{CellBuilder, CellParser};
use tonstruct::{FromCell, ToCell};

/// Lets for example our custom type will store RGB data
pub struct RGB(u8, u8, u8);

impl FromCell for RGB {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        // e.g.
        let r = parser.load_u8(8)?;
        let g = parser.load_u8(8)?;
        let b = parser.load_u8(8)?;

        Ok(RGB(r, g, b))
    }
}

impl ToCell for RGB {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_u8(8, self.0)?;
        builder.store_u8(8, self.1)?;
        builder.store_u8(8, self.2)?;

        Ok(builder)
    }
}

#[derive(FromCell, ToCell)]
pub struct Message {
    rgb: RGB,
}

fn main() {
    // Now you can serialize this type
    let rgb = RGB(255, 0, 0);
    let cell = rgb.to_cell().unwrap();

    // Or deserialize
    let _rgb = RGB::from_cell(cell).unwrap();
}
