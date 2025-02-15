//! TON transaction body *de*serialization crate
//!
//! These docs are not 100% ready. You can help us to improve it
//!
//! ## Example
//!
//! ## Deserialization from Cell
//!
//! To parse transaction body you need to derive `FromCell` macro.
//! In example below we parse transaction body from jetton transfer
//!
//! ```rust
//! use tonstruct::{FromCell, fields::{Uint, Coins, Address, Int, CellRef, Comment}};
//!
//! #[derive(FromCell, Debug, PartialEq)]
//! struct ForwardPayload {
//!     is_right: bool,
//!     text_comment: CellRef<Comment>,
//! }
//!
//! #[derive(FromCell, Debug, PartialEq)]
//! struct CustomPayload {}
//!
//! #[derive(FromCell, Debug, PartialEq)]
//! struct Message {
//!     op_code: Uint<32>,
//!     query_id: Uint<64>,
//!     amount: Coins,
//!     destination: Address,
//!     response_destination: Address,
//!     custom_payload: Option<CustomPayload>,
//!     forward_ton_amount: Coins,
//!     forward_payload: ForwardPayload,
//! }
//!
//! fn parse_body(cell: tonlib_core::cell::Cell) -> Message {
//!     <Message as FromCell>::from_cell(cell).unwrap()
//! }
//! ```
//!
//! ## Serialization to Cell
//!
//! ```rust
//! use tonstruct::{ToCell, fields::{Comment}};
//!
//! #[derive(ToCell)]
//! struct Message {
//!     field: Comment
//! }
//!
//! fn struct_to_cell(message: Message) -> tonlib_core::cell::Cell {
//!     message.to_cell().unwrap()
//! }
//! ```
//!
//! # Implemented TON types
//!
//! To see all implemented TON types and its documentation open [`fields`] page

mod from_cell;
mod to_cell;
mod types;
mod utils;

pub use from_cell::FromCell;
pub use to_cell::ToCell;
pub use tonstruct_proc_macro::{FromCell, ToCell};
pub mod fields {
    //! # Implemented TON types
    //!
    //! ## Boolean
    //!
    //! TON boolean implemented using built-in Rust [`bool`]
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     boolean_field: bool,
    //! }
    //! ```
    //!
    //! ## [`Int`]
    //!
    //! TON integer fields implemented using custom wrapper structure [`Int`]
    //!
    //! Bits count is specified by adding const generic parameter (e.g. 32 bit integer would be look like `Int<32>`)
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Int;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     integer_field: Int<32>,
    //! }
    //! ```
    //! ## [`Uint`]
    //!
    //! Unsigned integer type is similar to regular Int type.
    //!
    //! Bits count is also specified by adding const generic parameter (e.g. 32 bit unsigned integer would be look like `Uint<32>`)
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Uint;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     unsigned_integer_field: Uint<32>,
    //! }
    //! ```
    //!
    //! ## [`Coins`]
    //!
    //! Coins is just like integers but without bits count
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Coins;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     unsigned_integer_field: Coins,
    //! }
    //! ```
    //!
    //! ## [`Address`]
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Address;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     address_field: Address,
    //! }
    //! ```
    //!
    //! ## Optional field
    //!
    //! Optional fields implemented using build-in [`Option`] enum that checks 1 bit before parsing.
    //! If that bit is `0b01` so enum will be resolved as [`Some`].
    //! Otherwise, as [`None`]
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Address;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     optional_address_field: Option<Address>,
    //! }
    //! ```
    //!
    //! ## String
    //!
    //! Regular string (simple utf8 byte sequence) is implemented using built-in [`String`] type.
    //!
    //! See below other string types!
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     string_field: String,
    //! }
    //! ```
    //!
    //! ## [`Comment`]
    //!
    //! It's just a simple string but prefixed with 4 null bytes
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::Comment;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     comment_string_field: Comment,
    //! }
    //! ```
    //!
    //! # Structures
    //!
    //! ## Simple
    //!
    //! You can use simple sub structures. Its fields will be parsed sequentially without loading reference Cell.
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Sub {
    //!     // Some fields
    //! }
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     substruct: Sub,
    //! }
    //! ```
    //!
    //! ## [`CellRef`]
    //!
    //! This wrapper is used when you need to parse structure in separated referenced Cell
    //!
    //! ```rust
    //! use tonstruct::{ToCell, FromCell};
    //! use tonstruct::fields::CellRef;
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Sub {
    //!     // Some fields
    //! }
    //!
    //! #[derive(ToCell, FromCell)]
    //! struct Message {
    //!     substruct: CellRef<Sub>,
    //! }
    //! ```
    pub use crate::types::address::Address;
    pub use crate::types::cell_ref::CellRef;
    pub use crate::types::coins::Coins;
    pub use crate::types::comment::Comment;
    pub use crate::types::int::Int;
    pub use crate::types::uint::Uint;
}
