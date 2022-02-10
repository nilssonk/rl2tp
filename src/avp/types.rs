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

#[derive(Clone, Debug, PartialEq)]
pub struct HostName {}
#[derive(Clone, Debug, PartialEq)]
pub struct VendorName {}
#[derive(Clone, Debug, PartialEq)]
pub struct AssignedTunnelId {}
#[derive(Clone, Debug, PartialEq)]
pub struct ReceiveWindowSize {}
#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {}
#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeResponse {}
#[derive(Clone, Debug, PartialEq)]
pub struct Q931CauseCode {}
#[derive(Clone, Debug, PartialEq)]
pub struct AssignedSessionId {}
#[derive(Clone, Debug, PartialEq)]
pub struct CallSerialNumber {}
#[derive(Clone, Debug, PartialEq)]
pub struct MinimumBps {}
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

impl HostName {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl VendorName {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl AssignedTunnelId {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl ReceiveWindowSize {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl Challenge {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl ChallengeResponse {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl Q931CauseCode {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl AssignedSessionId {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl CallSerialNumber {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
impl MinimumBps {
    pub fn from(_data: &[u8]) -> ResultStr<Self> {
        unimplemented!();
    }
}
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
