//! A Rust implementation of the L2TP protocol.
//
//! Provides the full set of protocol messages required to speak L2TP, together
//! with some utility traits used to read and write them.

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

pub mod avp;
pub mod common;
mod message;
pub use message::*;
