use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

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

static MESSAGE_CODE_TO_TYPE: phf::Map<u16, MessageType> = phf_map! {
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
    const ATTRIBUTE_TYPE: u16 = 0;
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete MessageType AVP payload encountered");
        }
        let id = unsafe { reader.read_u16_be_unchecked() };

        match MESSAGE_CODE_TO_TYPE.get(&id) {
            Some(&t) => Ok(t),
            None => Err("Unknown MessageType AVP encountered"),
        }
    }

    const fn get_code(&self) -> u16 {
        match self {
            StartControlConnectionRequest => 1u16,
            StartControlConnectionReply => 2u16,
            StartControlConnectionConnected => 3u16,
            StopControlConnectionNotification => 4u16,
            Hello => 6u16,
            OutgoingCallRequest => 7u16,
            OutgoingCallReply => 8u16,
            OutgoingCallConnected => 9u16,
            IncomingCallRequest => 10u16,
            IncomingCallReply => 11u16,
            IncomingCallConnected => 12u16,
            CallDisconnectNotify => 14u16,
            WanErrorNotify => 15u16,
            SetLinkInfo => 16u16,
        }
    }
}

impl QueryableAVP for MessageType {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for MessageType {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_u16_be_unchecked(self.get_code())
    }
}
