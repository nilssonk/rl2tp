use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

use phf::phf_map;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    const LENGTH: usize = 2;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }
        let id = unsafe { reader.read_u16_be_unchecked() };

        match MESSAGE_CODE_TO_TYPE.get(&id) {
            Some(&t) => Ok(t),
            None => Err(DecodeError::UnknownMessageType(id)),
        }
    }

    #[inline]
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
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for MessageType {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be(self.get_code())
    }
}
