use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    StartControlConnectionRequest,
    StartControlConnectionReply,
    StartControlConnectionConnected,
    StopControlConnectionNotification,
    Hello,
    OutgoingCallRequest,
    OutgoingCallReply,
    OutgoingCallConnected,
    IncomingCallRequest,
    IncomingCallReply,
    IncomingCallConnected,
    CallDisconnectNotify,
    WanErrorNotify,
    SetLinkInfo,
}

use MessageType::*;

static MESSAGE_TYPES: phf::Map<u16, MessageType> = phf_map! {
    1u16 => StartControlConnectionRequest,
    2u16 => StartControlConnectionReply,
    3u16 => StartControlConnectionConnected,
    4u16 => StopControlConnectionNotification,
    6u16 => Hello,
    7u16 => OutgoingCallRequest,
    8u16 => OutgoingCallReply,
    9u16 => OutgoingCallConnected,
    10u16 => IncomingCallRequest,
    11u16 => IncomingCallReply,
    12u16 => IncomingCallConnected,
    14u16 => CallDisconnectNotify,
    15u16 => WanErrorNotify,
    16u16 => SetLinkInfo,
};

impl MessageType {
    const LENGTH: u16 = 2;

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete MessageType AVP payload encountered");
        }
        let id = unsafe { reader.read_u16_be_unchecked() };

        match MESSAGE_TYPES.get(&id) {
            Some(&t) => Ok(t),
            None => Err("Unknown MessageType AVP encountered"),
        }
    }
}

impl QueryableAVP for MessageType {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}
