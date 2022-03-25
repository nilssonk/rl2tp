mod flags;
use flags::Flags;

pub mod types;

use crate::common::{Reader, ResultStr, SliceReader, Writer};
use enum_dispatch::enum_dispatch;
use phf::phf_map;

#[enum_dispatch]
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
    SequencingRequired(types::SequencingRequired),
    InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq),
    LastSentLcpConfReq(types::LastSentLcpConfReq),
    LastReceivedLcpConfReq(types::LastReceivedLcpConfReq),
    ProxyAuthenType(types::ProxyAuthenType),
    ProxyAuthenName(types::ProxyAuthenName),
    ProxyAuthenChallenge(types::ProxyAuthenChallenge),
    ProxyAuthenId(types::ProxyAuthenId),
    ProxyAuthenResponse(types::ProxyAuthenResponse),
    CallErrors(types::CallErrors),
    Accm(types::Accm),
    Hidden(types::Hidden),
}

#[enum_dispatch(AVP)]
pub trait QueryableAVP {
    fn get_length(&self) -> u16;
}

#[enum_dispatch(AVP)]
pub trait WritableAVP {
    unsafe fn write(&self, writer: &mut dyn Writer);
}

use AVP::*;

type DecodeFunction = for<'a> fn(Box<dyn Reader<'a> + 'a>) -> ResultStr<AVP>;
static AVP_CODES: phf::Map<u16, DecodeFunction> = phf_map! {
    0u16 => |reader| Ok(MessageType(types::MessageType::try_read(reader)?)),
    1u16 => |reader| Ok(ResultCode(types::ResultCode::try_read(reader)?)),
    2u16 => |reader| Ok(ProtocolVersion(types::ProtocolVersion::try_read(reader)?)),
    3u16 => |reader| Ok(FramingCapabilities(types::FramingCapabilities::try_read(reader)?)),
    4u16 => |reader| Ok(BearerCapabilities(types::BearerCapabilities::try_read(reader)?)),
    5u16 => |reader| Ok(TieBreaker(types::TieBreaker::try_read(reader)?)),
    6u16 => |reader| Ok(FirmwareRevision(types::FirmwareRevision::try_read(reader)?)),
    7u16 => |reader| Ok(HostName(types::HostName::try_read(reader)?)),
    8u16 => |reader| Ok(VendorName(types::VendorName::try_read(reader)?)),
    9u16 => |reader| Ok(AssignedTunnelId(types::AssignedTunnelId::try_read(reader)?)),
    10u16 => |reader| Ok(ReceiveWindowSize(types::ReceiveWindowSize::try_read(reader)?)),
    11u16 => |reader| Ok(Challenge(types::Challenge::try_read(reader)?)),
    12u16 => |reader| Ok(Q931CauseCode(types::Q931CauseCode::try_read(reader)?)),
    13u16 => |reader| Ok(ChallengeResponse(types::ChallengeResponse::try_read(reader)?)),
    14u16 => |reader| Ok(AssignedSessionId(types::AssignedSessionId::try_read(reader)?)),
    15u16 => |reader| Ok(CallSerialNumber(types::CallSerialNumber::try_read(reader)?)),
    16u16 => |reader| Ok(MinimumBps(types::MinimumBps::try_read(reader)?)),
    17u16 => |reader| Ok(MaximumBps(types::MaximumBps::try_read(reader)?)),
    18u16 => |reader| Ok(BearerType(types::BearerType::try_read(reader)?)),
    19u16 => |reader| Ok(FramingType(types::FramingType::try_read(reader)?)),
    21u16 => |reader| Ok(CalledNumber(types::CalledNumber::try_read(reader)?)),
    22u16 => |reader| Ok(CallingNumber(types::CallingNumber::try_read(reader)?)),
    23u16 => |reader| Ok(SubAddress(types::SubAddress::try_read(reader)?)),
    24u16 => |reader| Ok(TxConnectSpeed(types::TxConnectSpeed::try_read(reader)?)),
    25u16 => |reader| Ok(PhysicalChannelId(types::PhysicalChannelId::try_read(reader)?)),
    26u16 => |reader| Ok(InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq::try_read(reader)?)),
    27u16 => |reader| Ok(LastSentLcpConfReq(types::LastSentLcpConfReq::try_read(reader)?)),
    28u16 => |reader| Ok(LastReceivedLcpConfReq(types::LastReceivedLcpConfReq::try_read(reader)?)),
    29u16 => |reader| Ok(ProxyAuthenType(types::ProxyAuthenType::try_read(reader)?)),
    30u16 => |reader| Ok(ProxyAuthenName(types::ProxyAuthenName::try_read(reader)?)),
    31u16 => |reader| Ok(ProxyAuthenChallenge(types::ProxyAuthenChallenge::try_read(reader)?)),
    32u16 => |reader| Ok(ProxyAuthenId(types::ProxyAuthenId::try_read(reader)?)),
    33u16 => |reader| Ok(ProxyAuthenResponse(types::ProxyAuthenResponse::try_read(reader)?)),
    34u16 => |reader| Ok(CallErrors(types::CallErrors::try_read(reader)?)),
    35u16 => |reader| Ok(Accm(types::Accm::try_read(reader)?)),
    36u16 => |reader| Ok(RandomVector(types::RandomVector::try_read(reader)?)),
    37u16 => |reader| Ok(PrivateGroupId(types::PrivateGroupId::try_read(reader)?)),
    38u16 => |reader| Ok(RxConnectSpeed(types::RxConnectSpeed::try_read(reader)?)),
    39u16 => |reader| Ok(SequencingRequired(types::SequencingRequired::default())),
};

const N_HEADER_OCTETS: usize = 6;
const N_FLAG_LENGTH_VENDOR_OCTETS: usize = 4;
const ATTRIBUTE_TYPE_SIZE: usize = 2;

impl AVP {
    pub fn reveal(self, secret: &[u8], random_vector: &[u8]) -> ResultStr<Self> {
        match self {
            Self::Hidden(mut hidden) => {
                // The first 4 octets have been peeled off and we are guaranteed to have at least 6
                assert!(hidden.data.len() >= ATTRIBUTE_TYPE_SIZE);
                let attribute_type =
                    unsafe { SliceReader::from(&hidden.data).read_u16_be_unchecked() };

                const CHUNK_SIZE: usize = 16;
                let chunk_data = &mut hidden.data[ATTRIBUTE_TYPE_SIZE..];
                let n_chunks = chunk_data.len() / CHUNK_SIZE;

                if chunk_data.is_empty() {
                    return Err("Hidden AVP with empty payload encountered");
                }

                // The largest size is the size of the final intermediate value
                let buffer_length = ATTRIBUTE_TYPE_SIZE + secret.len() + random_vector.len();
                let mut buffer = Vec::with_capacity(buffer_length);

                if n_chunks > 1 {
                    // The shared secret is a prefix for all chunks except the first one, so set it once for the entire loop
                    buffer.extend_from_slice(secret);

                    // Loop over chunks in reverse order
                    for i in (1..n_chunks).rev() {
                        let prev_chunk_start = (i - 1) * CHUNK_SIZE;
                        let chunk_start = prev_chunk_start + CHUNK_SIZE;

                        // Retain only the prefix which is guaranteed to be the shared secret
                        buffer.truncate(secret.len());

                        // The intermediate value for a given chunk is MD5(secret + previous chunk)
                        buffer.extend_from_slice(&chunk_data[prev_chunk_start..chunk_start]);
                        let intermediate = md5::compute(&buffer);

                        // Decode with XOR
                        for j in 0..CHUNK_SIZE {
                            chunk_data[chunk_start + j] ^= intermediate[j];
                        }
                    }
                }

                // The final intermediate value is MD5(Attribute type + secret + RV)
                buffer.clear();
                buffer.extend_from_slice(&attribute_type.to_be_bytes());
                buffer.extend_from_slice(secret);
                buffer.extend_from_slice(random_vector);
                let intermediate = md5::compute(&buffer);

                // Decode with XOR
                for j in 0..CHUNK_SIZE {
                    chunk_data[j] ^= intermediate[j];
                }

                let reader = Box::new(SliceReader::from(chunk_data));

                AVP_CODES.get(&attribute_type).map_or_else(
                    || Err("Unknown hidden AVP encountered"),
                    |constructor| constructor(reader),
                )
            }
            _ => Ok(self),
        }
    }

    pub fn try_read_greedy<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> Vec<ResultStr<Self>> {
        let mut result = Vec::new();
        while !reader.is_empty() {
            // Note: Subsequent unsafe code depends on this check
            if reader.len() < N_HEADER_OCTETS {
                result.push(Err("Incomplete AVP header encountered"));
                break;
            }

            let (flags, length, vendor_id) =
                unsafe { Self::read_flags_length_vendor_unchecked(reader.as_mut()) };

            let payload_length = length as usize - N_FLAG_LENGTH_VENDOR_OCTETS;

            let avp = if vendor_id != 0 {
                Err("AVP with unsupported vendor ID encountered")
            } else if payload_length > reader.len() {
                Err("AVP with invalid length field encountered")
            } else if flags.is_hidden() {
                // Hidden AVP
                let hidden_data = reader
                    .subreader(length as usize)
                    .read_bytes(reader.len())
                    .unwrap_or_default();
                Ok(Self::Hidden(types::Hidden { data: hidden_data }))
            } else {
                // Regular AVP
                unsafe { Self::decode(reader.subreader(payload_length)) }
            };
            result.push(avp);
        }

        result
    }

    unsafe fn read_flags_length_vendor_unchecked(reader: &mut dyn Reader) -> (Flags, u16, u16) {
        assert!(reader.len() >= N_FLAG_LENGTH_VENDOR_OCTETS);

        // Flags and length share the first 2 octets
        let octet1 = reader.read_u8_unchecked();
        let octet2 = reader.read_u8_unchecked();
        let flags = Flags::from(octet1);
        let msb = (octet1 >> 6) as u16;
        let lsb = octet2 as u16;
        let length = msb << 8 | lsb;

        // The second 2 octets are the Vendor ID
        let vendor_id = reader.read_u16_be_unchecked();

        (flags, length, vendor_id)
    }

    unsafe fn decode<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        // The first 4 octets have been peeled off and we are guaranteed to have at least 6
        assert!(reader.len() >= ATTRIBUTE_TYPE_SIZE);

        let attribute_type = reader.read_u16_be_unchecked();

        AVP_CODES.get(&attribute_type).map_or_else(
            || Err("Unknown AVP encountered"),
            |constructor| constructor(reader),
        )
    }
}
