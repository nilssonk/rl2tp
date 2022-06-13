//! A Rust implementation of the L2TP protocol.
//
//! Provides the full set of protocol messages required to speak L2TP, together
//! with some utility traits used to read and write them.
//!
//! # Cargo Features
//! * `benchmarks` - Enable benchmarking with [criterion.rs](https://github.com/bheisler/criterion.rs).

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

pub mod common;

pub use common::Reader;
pub use common::Writer;

mod message;
pub use message::*;
