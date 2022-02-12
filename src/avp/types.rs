use crate::common::ResultStr;

mod message_type;
pub use message_type::*;

mod random_vector;
pub use random_vector::*;

pub mod result_code;
pub use result_code::ResultCode;

mod protocol_version;
pub use protocol_version::*;

mod framing_capabilities;
pub use framing_capabilities::*;

mod bearer_capabilities;
pub use bearer_capabilities::*;

mod tie_breaker;
pub use tie_breaker::*;

mod firmware_revision;
pub use firmware_revision::*;

mod host_name;
pub use host_name::*;

mod vendor_name;
pub use vendor_name::*;

mod assigned_tunnel_id;
pub use assigned_tunnel_id::*;

mod receive_window_size;
pub use receive_window_size::*;

mod challenge;
pub use challenge::*;

mod challenge_response;
pub use challenge_response::*;

mod q931_cause_code;
pub use q931_cause_code::*;

mod assigned_session_id;
pub use assigned_session_id::*;

mod call_serial_number;
pub use call_serial_number::*;

mod minimum_bps;
pub use minimum_bps::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MaximumBps {}
#[derive(Clone, Debug, PartialEq)]
pub struct BearerType {}
#[derive(Clone, Debug, PartialEq)]
pub struct FramingType {}
#[derive(Clone, Debug, PartialEq)]
pub struct CalledNumber {}
#[derive(Clone, Debug, PartialEq)]
pub struct CallingNumber {}
#[derive(Clone, Debug, PartialEq)]
pub struct SubAddress {}
#[derive(Clone, Debug, PartialEq)]
pub struct TxConnectSpeed {}
#[derive(Clone, Debug, PartialEq)]
pub struct RxConnectSpeed {}
#[derive(Clone, Debug, PartialEq)]
pub struct PhysicalChannelId {}
#[derive(Clone, Debug, PartialEq)]
pub struct PrivateGroupId {}
#[derive(Clone, Debug, PartialEq)]
pub struct SequencingRequired {}

impl MaximumBps {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl BearerType {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl FramingType {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl CalledNumber {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl CallingNumber {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl SubAddress {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl TxConnectSpeed {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl RxConnectSpeed {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl PhysicalChannelId {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl PrivateGroupId {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl SequencingRequired {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
