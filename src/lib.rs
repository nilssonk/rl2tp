//! A Rust implementation of the L2TP protocol.
//
//! This crate provides the full set of protocol messages required to speak L2TP, together
//! with some utility traits used to read and write them.
//!
//! # Examples
//! ## Read L2TP data from a buffer
//! ```
//! use rl2tp::{Message, common::SliceReader};
//! let buffer = vec![
//!        0x13, 0x20, // Flags
//!        0x00, 0x14, // Length
//!        0x00, 0x02, // Tunnel ID
//!        0x00, 0x03, // Session ID
//!        0x00, 0x04, // Ns
//!        0x00, 0x05, // Nr
//!        // AVP Payload
//!        0x00, 0x08, // Flags and Length
//!        0x00, 0x00, // Vendor ID
//!        0x00, 0x00, // Attribute Type (Message Type)
//!        0x00, 0x01, // Type 1 (StartControlConnectionRequest)
//! ];
//!
//! let mut r = SliceReader::from(&buffer);
//! let msg = Message::try_read(&mut r).unwrap();
//!
//! match msg {
//!     Message::Data(_) => (),
//!     Message::Control(_) => (),
//! }
//! ```
//!
//!
//! ## Write an L2TP control message to a `Vec`
//! ```
//! use rl2tp::{Message, common::VecWriter, avp::{AVP, types::MessageType}};
//!
//! let mut w = VecWriter::new();
//! let msg = Message::Control(rl2tp::ControlMessage {
//!     length: 0,
//!     tunnel_id: 5,
//!     session_id: 0,
//!     ns: 0,
//!     nr: 0,
//!     avps: vec![
//!         AVP::MessageType(MessageType::StartControlConnectionConnected),
//!     ],
//! });
//! unsafe { msg.write(&mut w) };
//! ```
//!
//! ## Write an L2TP data message to a buffer
//!
//! ```
//!
//! use rl2tp::{Message, common::VecWriter};
//!
//! let data = vec![0xde,0xad,0xbe,0xef];
//!
//! let mut w = rl2tp::common::VecWriter::new();
//! let msg = rl2tp::Message::Data(rl2tp::DataMessage {
//!     is_prioritized: false,
//!     length: None,
//!     tunnel_id: 5,
//!     session_id: 0,
//!     ns_nr: None,
//!     offset: None,
//!     data: &data,
//! });
//! unsafe { msg.write(&mut w) };
//! ```
//!
//! # Cargo Features
//! * `benchmarks` - Enable benchmarking with [criterion.rs](https://github.com/bheisler/criterion.rs).

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

pub mod common;

pub use common::Reader;
pub use common::Writer;

mod message;
pub use message::*;
