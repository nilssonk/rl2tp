mod flags;
pub mod types;

use crate::common::{read_u16_be_unchecked, ResultStr};
use flags::Flags;
use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum AVP {
    MessageType(types::MessageType),
    RandomVector(types::RandomVector),
    ResultCode(types::ResultCode),
    ProtocolVersion(types::ProtocolVersion),
    FramingCapabilities(types::FramingCapabilities),
    BearerCapabilities(types::BearerCapabilities),
    TieBreaker(types::TieBreaker),
    FirmwareRevision(types::FirmwareRevision),
    HostName(types::HostName),
    VendorName(types::VendorName),
    AssignedTunnelId(types::AssignedTunnelId),
    ReceiveWindowSize(types::ReceiveWindowSize),
    Challenge(types::Challenge),
    ChallengeResponse(types::ChallengeResponse),
    Q931CauseCode(types::Q931CauseCode),
    AssignedSessionId(types::AssignedSessionId),
    CallSerialNumber(types::CallSerialNumber),
    MinimumBps(types::MinimumBps),
    MaximumBps(types::MaximumBps),
    BearerType(types::BearerType),
    FramingType(types::FramingType),
    CalledNumber(types::CalledNumber),
    CallingNumber(types::CallingNumber),
    SubAddress(types::SubAddress),
    TxConnectSpeed(types::TxConnectSpeed),
    RxConnectSpeed(types::RxConnectSpeed),
    PhysicalChannelId(types::PhysicalChannelId),
    PrivateGroupId(types::PrivateGroupId),
    SequencingRequired,
    InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq),
    LastSentLcpConfReq(types::LastSentLcpConfReq),
    LastReceivedLcpConfReq(types::LastReceivedLcpConfReq),
}

use AVP::*;

type DecodeFunction = fn(&[u8]) -> ResultStr<AVP>;
static AVP_CODES: phf::Map<u16, DecodeFunction> = phf_map! {
    0u16 => |data| Ok(MessageType(types::MessageType::from(data)?)),
    1u16 => |data| Ok(ResultCode(types::ResultCode::from(data)?)),
    2u16 => |data| Ok(ProtocolVersion(types::ProtocolVersion::from(data)?)),
    3u16 => |data| Ok(FramingCapabilities(types::FramingCapabilities::from(data)?)),
    4u16 => |data| Ok(BearerCapabilities(types::BearerCapabilities::from(data)?)),
    5u16 => |data| Ok(TieBreaker(types::TieBreaker::from(data)?)),
    6u16 => |data| Ok(FirmwareRevision(types::FirmwareRevision::from(data)?)),
    7u16 => |data| Ok(HostName(types::HostName::from(data)?)),
    8u16 => |data| Ok(VendorName(types::VendorName::from(data)?)),
    9u16 => |data| Ok(AssignedTunnelId(types::AssignedTunnelId::from(data)?)),
    10u16 => |data| Ok(ReceiveWindowSize(types::ReceiveWindowSize::from(data)?)),
    11u16 => |data| Ok(Challenge(types::Challenge::from(data)?)),
    12u16 => |data| Ok(Q931CauseCode(types::Q931CauseCode::from(data)?)),
    13u16 => |data| Ok(ChallengeResponse(types::ChallengeResponse::from(data)?)),
    14u16 => |data| Ok(AssignedSessionId(types::AssignedSessionId::from(data)?)),
    15u16 => |data| Ok(CallSerialNumber(types::CallSerialNumber::from(data)?)),
    16u16 => |data| Ok(MinimumBps(types::MinimumBps::from(data)?)),
    17u16 => |data| Ok(MaximumBps(types::MaximumBps::from(data)?)),
    18u16 => |data| Ok(BearerType(types::BearerType::from(data)?)),
    19u16 => |data| Ok(FramingType(types::FramingType::from(data)?)),
    21u16 => |data| Ok(CalledNumber(types::CalledNumber::from(data)?)),
    22u16 => |data| Ok(CallingNumber(types::CallingNumber::from(data)?)),
    23u16 => |data| Ok(SubAddress(types::SubAddress::from(data)?)),
    24u16 => |data| Ok(TxConnectSpeed(types::TxConnectSpeed::from(data)?)),
    25u16 => |data| Ok(PhysicalChannelId(types::PhysicalChannelId::from(data)?)),
    26u16 => |data| Ok(InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq::from(data)?)),
    27u16 => |data| Ok(LastSentLcpConfReq(types::LastSentLcpConfReq::from(data)?)),
    28u16 => |data| Ok(LastReceivedLcpConfReq(types::LastReceivedLcpConfReq::from(data)?)),
    36u16 => |data| Ok(RandomVector(types::RandomVector::from(data)?)),
    37u16 => |data| Ok(PrivateGroupId(types::PrivateGroupId::from(data)?)),
    38u16 => |data| Ok(RxConnectSpeed(types::RxConnectSpeed::from(data)?)),
    39u16 => |data| Ok(SequencingRequired)
};

impl AVP {
    pub fn from_bytes_greedy(input: &[u8]) -> (Vec<ResultStr<Self>>, ResultStr<()>) {
        let mut avp_start_offset = 0;
        let mut result = Vec::new();
        while avp_start_offset < input.len() {
            const N_HEADER_OCTETS: usize = 6;
            const N_FLAG_LENGTH_OCTETS: usize = 2;

            let (flags, length) = Self::read_flags_length(
                &input[avp_start_offset..avp_start_offset + N_FLAG_LENGTH_OCTETS],
            );

            if input.len() < avp_start_offset + N_HEADER_OCTETS {
                result.push(Err("Incomplete AVP header encountered"));
                break;
            }

            let mut has_error = false;

            let avp_end_offset = avp_start_offset + length as usize;
            if avp_end_offset > input.len() {
                has_error = true;
                result.push(Err("Invalid AVP length field encountered"));
            } else if flags.is_hidden() {
                // @TODO: Support hidden
                has_error = true;
                result.push(Err("AVP with hidden flag encountered - ignoring"));
            }

            if !has_error {
                let decode_start_offset = avp_start_offset + N_FLAG_LENGTH_OCTETS;
                let avp = Self::decode(&input[decode_start_offset..avp_end_offset]);
                result.push(avp);
            }

            avp_start_offset = avp_end_offset;
        }

        if let Some(first) = result.first() {
            match first {
                Ok(MessageType(_)) => (),
                _ => return (result, Err("First AVP is not a MessageType AVP")),
            }
        }

        (result, Ok(()))
    }

    fn read_flags_length(input: &[u8]) -> (Flags, u16) {
        let flags = Flags::from(input[0]);
        let msb = (input[0] >> 6) as u16;
        let lsb = input[1] as u16;
        let length = msb << 8 | lsb;

        (flags, length)
    }

    fn decode(input: &[u8]) -> ResultStr<Self> {
        assert!(input.len() >= 4);

        let mut offset = 0;

        let vendor_id = unsafe { read_u16_be_unchecked(&input[offset..]) };
        offset += 2;
        if vendor_id != 0 {
            return Err("Unsupported AVP vendor ID encountered");
        }

        let attribute_type = unsafe { read_u16_be_unchecked(&input[offset..]) };
        offset += 2;
        match AVP_CODES.get(&attribute_type) {
            Some(constructor) => constructor(&input[offset..]),
            None => Err("Could not decode mandatory AVP"),
        }
    }
}
