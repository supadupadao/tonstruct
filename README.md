# TonStruct

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/supadupadao/tonstruct/ci.yml?label=CI)](https://github.com/supadupadao/tonstruct/actions)
[![GitHub License](https://img.shields.io/github/license/supadupadao/tonstruct)](https://github.com/supadupadao/tonstruct/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/tonstruct)](https://crates.io/crates/tonstruct)
[![Codecov](https://img.shields.io/codecov/c/github/supadupadao/tonstruct)](https://app.codecov.io/gh/supadupadao/tonstruct)
[![Work in progress](https://img.shields.io/badge/WORK%20IN%20PROGRESS-DO%20NOT%20USE%20IN%20PRODUCTION-ff0000)](https://github.com/supadupadao/tonstruct/issues)

ℹ️ The Open Network *de*serialization crate for Rust language.

❤️ Any contributions are welcome. Feel free to open pull requests, issues, bug reports, feature proposals or anything
else

## Example

To parse transaction body you need to derive `FromCell` macro.
In example below we parse transaction body from jetton transfer

```rust
use tonstruct::{FromCell, fields::{Uint, Coins, Address, Int, CellRef, Comment}};

#[derive(FromCell, Debug, PartialEq)]
struct ForwardPayload {
    is_right: bool,
    text_comment: CellRef<Comment>,
}

#[derive(FromCell, Debug, PartialEq)]
struct JettonTransfer {
    op_code: Uint<32>,
    query_id: Uint<64>,
    amount: Coins,
    destination: Address,
    response_destination: Address,
    custom_payload: Option<Int<0>>,
    forward_ton_amount: Coins,
    forward_payload: ForwardPayload,
}

fn main() {
    // Transaction body Cell
    let cell = ...;
    // Parsed body
    let message = <Message as FromCell>::from_cell(cell).unwrap();
}
```

To serialize structure into `Cell` use derived macro `ToCell` and call `to_cell()` method.

```rust
use tonstruct::ToCell;

#[derive(ToCell)]
struct JettonTransfer {
    /// Fields
}

fn main() {
    // Message body
    let message = Message {
        /// Fields 
    };

    let cell = message.to_cell().unwrap();
    /// OR
    let cell = ToCell::to_cell(&message).unwrap();
}
```