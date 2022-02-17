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

mod maximum_bps;
pub use maximum_bps::*;

mod bearer_type;
pub use bearer_type::*;

mod framing_type;
pub use framing_type::*;

mod called_number;
pub use called_number::*;

mod calling_number;
pub use calling_number::*;

mod sub_address;
pub use sub_address::*;

mod tx_connect_speed;
pub use tx_connect_speed::*;

mod rx_connect_speed;
pub use rx_connect_speed::*;

mod physical_channel_id;
pub use physical_channel_id::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PrivateGroupId {}
#[derive(Clone, Debug, PartialEq)]
pub struct SequencingRequired {}

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
