# TonStruct

[![Work in progress](https://img.shields.io/badge/WORK%20IN%20PROGRESS-DO%20NOT%20USE%20IN%20PRODUCTION-ff0000)](https://github.com/supadupadao/tonstruct/issues)

ℹ️ The Open Network *de*serialization crate for Rust language.

❤️ Any contributions are welcome. Feel free to open pull requests, issues, bug reports, feature proposals or anything
else

## Example

To parse transaction body you need to derive `FromCell` macro. In example below we parse transaction body from [
`8917...cb83`](https://tonviewer.com/transaction/8917a2f87f0b9ec52673b868a7ae111c19654f5b58961b1f7034f384dc5bcb83)
jetton transfer.

```rust
use tonstruct::{FromCell, Uint, Coins, Address};

#[derive(FromCell, Debug, PartialEq)]
struct Message {
    op_code: Uint<32>,
    query_id: Uint<64>,
    amount: Coins,
    destination: Address,
    response_destination: Address,
    custom_payload: Option<Int<0>>,
    forward_ton_amount: Coins,
}

fn main() {
    // Transaction body Cell
    let cell = ...;
    // Parsed body
    let message = <Message as FromCell>::from_cell(cell).unwrap();
}
```