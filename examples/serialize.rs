//! In this example we will convert Rust structure into TON Cell

use tonstruct::fields::{Coins, Int, Uint};
use tonstruct::ToCell;

#[derive(ToCell)]
struct Message {
    field_a: String,
    field_b: bool,
    field_c: Int<13>,  // 13 is bits count
    field_d: Uint<37>, // 37 is bits count
    field_e: Coins,
}

fn main() {
    let message = Message {
        field_a: "You can use built-in string".to_string(),
        field_b: true,
        field_c: Int::from(6i32),
        field_d: Uint::from(9u32),
        field_e: Coins::from(100500u32),
    };

    let _ = message.to_cell().unwrap();
}
