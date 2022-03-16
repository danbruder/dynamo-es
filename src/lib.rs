#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]
//! # dynamo-es
//!
//! > A DynamoDb implementation of the `EventStore` trait in [cqrs-es](https://crates.io/crates/persist-es).
//!
pub use crate::cqrs::*;
pub use crate::event_repository::*;
pub use crate::types::*;
pub use crate::view_repository::*;

mod cqrs;
mod error;
mod event_repository;
mod helpers;
mod testing;
mod types;
mod view_repository;
