use crate::avp::avp_name;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DecodeError {
    #[error("Incomplete AVP ({})", avp_name(*.0))]
    IncompleteAVP(u16),

    #[error("MessageType AVP with unknown message type ({0})")]
    UnknownMessageType(u16),

    #[error("AVP ({}) with invalid UTF-8 string payload", avp_name(*.0))]
    InvalidUtf8(u16),

    #[error("Unknown error type ({0}) in ResultCode AVP")]
    InvalidResultCodeErrorType(u16),

    #[error("Read error when parsing AVP ({})", avp_name(*.0))]
    AVPReadError(u16),

    #[error("AVP with invalid length ({0})")]
    InvalidAVPLength(u16),

    #[error("AVP with unknown type ({0})")]
    UnknownAvp(u16),

    #[error("Hidden AVP with empty payload")]
    EmptyHiddenAVP,

    #[error("Hidden AVP with invalid alignment")]
    MisalignedHiddenAVP,

    #[error("Hidden AVP with invalid original length ({0})")]
    InvalidOriginalAVPLength(u16),

    #[error("AVP with unsupported vendor ID ({0}) encountered")]
    UnsupportedVendorId(u16),

    #[error("Message with invalid version field ({0})")]
    InvalidVersion(u8),

    #[error("Message with invalid reserved bits")]
    InvalidReservedBits,

    #[error("Message with incomplete flags field")]
    IncompleteFlags,

    #[error("Message with invalid offset ({0})")]
    InvalidOffset(u16),

    #[error("Incomplete data message header")]
    IncompleteDataMessageHeader,

    #[error("Incomplete data message payload")]
    IncompleteDataMessagePayload,

    #[error("Empty data message payload")]
    EmptyDataMessagePayload,

    #[error("Read error when parsing message")]
    MessageReadError,

    #[error("Control message with forbidden message priority present")]
    ForbiddenControlMessagePriority,

    #[error("Control message with forbidden offset present")]
    ForbiddenControlMessageOffset,

    #[error("Control message without required length field")]
    ControlMessageWithoutLength,

    #[error("Control message without required NsNr field")]
    ControlMessageWithoutNsNr,

    #[error("Incomplete control message header")]
    IncompleteControlMessageHeader,

    #[error("Incomplete control message payload")]
    IncompleteControlMessagePayload,

    #[error("First AVP of control message is not MessageType")]
    ControlMessageTypeNotFirst,
}
