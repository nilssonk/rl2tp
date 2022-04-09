#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

pub mod avp;
pub mod common;

mod message;
pub use message::*;
