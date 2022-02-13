mod flags;
use flags::Flags;

pub mod types;

use crate::common::{read_u16_be_unchecked, ResultStr};
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
    ProxyAuthenType(types::ProxyAuthenType),
    ProxyAuthenName(types::ProxyAuthenName),
    ProxyAuthenChallenge(types::ProxyAuthenChallenge),
    ProxyAuthenId(types::ProxyAuthenId),
    ProxyAuthenResponse(types::ProxyAuthenResponse),
    CallErrors(types::CallErrors),
    Accm(types::Accm),
    Hidden(Vec<u8>),
}

use AVP::*;

type DecodeFunction = fn(&[u8]) -> ResultStr<AVP>;
static AVP_CODES: phf::Map<u16, DecodeFunction> = phf_map! {
    0u16 => |data| Ok(MessageType(types::MessageType::try_from_bytes(data)?)),
    1u16 => |data| Ok(ResultCode(types::ResultCode::try_from_bytes(data)?)),
    2u16 => |data| Ok(ProtocolVersion(types::ProtocolVersion::try_from_bytes(data)?)),
    3u16 => |data| Ok(FramingCapabilities(types::FramingCapabilities::try_from_bytes(data)?)),
    4u16 => |data| Ok(BearerCapabilities(types::BearerCapabilities::try_from_bytes(data)?)),
    5u16 => |data| Ok(TieBreaker(types::TieBreaker::try_from_bytes(data)?)),
    6u16 => |data| Ok(FirmwareRevision(types::FirmwareRevision::try_from_bytes(data)?)),
    7u16 => |data| Ok(HostName(types::HostName::try_from_bytes(data)?)),
    8u16 => |data| Ok(VendorName(types::VendorName::try_from_bytes(data)?)),
    9u16 => |data| Ok(AssignedTunnelId(types::AssignedTunnelId::try_from_bytes(data)?)),
    10u16 => |data| Ok(ReceiveWindowSize(types::ReceiveWindowSize::try_from_bytes(data)?)),
    11u16 => |data| Ok(Challenge(types::Challenge::try_from_bytes(data)?)),
    12u16 => |data| Ok(Q931CauseCode(types::Q931CauseCode::try_from_bytes(data)?)),
    13u16 => |data| Ok(ChallengeResponse(types::ChallengeResponse::try_from_bytes(data)?)),
    14u16 => |data| Ok(AssignedSessionId(types::AssignedSessionId::try_from_bytes(data)?)),
    15u16 => |data| Ok(CallSerialNumber(types::CallSerialNumber::try_from_bytes(data)?)),
    16u16 => |data| Ok(MinimumBps(types::MinimumBps::try_from_bytes(data)?)),
    17u16 => |data| Ok(MaximumBps(types::MaximumBps::try_from_bytes(data)?)),
    18u16 => |data| Ok(BearerType(types::BearerType::try_from_bytes(data)?)),
    19u16 => |data| Ok(FramingType(types::FramingType::try_from_bytes(data)?)),
    21u16 => |data| Ok(CalledNumber(types::CalledNumber::try_from_bytes(data)?)),
    22u16 => |data| Ok(CallingNumber(types::CallingNumber::try_from_bytes(data)?)),
    23u16 => |data| Ok(SubAddress(types::SubAddress::try_from_bytes(data)?)),
    24u16 => |data| Ok(TxConnectSpeed(types::TxConnectSpeed::try_from_bytes(data)?)),
    25u16 => |data| Ok(PhysicalChannelId(types::PhysicalChannelId::try_from_bytes(data)?)),
    26u16 => |data| Ok(InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq::try_from_bytes(data)?)),
    27u16 => |data| Ok(LastSentLcpConfReq(types::LastSentLcpConfReq::try_from_bytes(data)?)),
    28u16 => |data| Ok(LastReceivedLcpConfReq(types::LastReceivedLcpConfReq::try_from_bytes(data)?)),
    29u16 => |data| Ok(ProxyAuthenType(types::ProxyAuthenType::try_from_bytes(data)?)),
    30u16 => |data| Ok(ProxyAuthenName(types::ProxyAuthenName::try_from_bytes(data)?)),
    31u16 => |data| Ok(ProxyAuthenChallenge(types::ProxyAuthenChallenge::try_from_bytes(data)?)),
    32u16 => |data| Ok(ProxyAuthenId(types::ProxyAuthenId::try_from_bytes(data)?)),
    33u16 => |data| Ok(ProxyAuthenResponse(types::ProxyAuthenResponse::try_from_bytes(data)?)),
    34u16 => |data| Ok(CallErrors(types::CallErrors::try_from_bytes(data)?)),
    35u16 => |data| Ok(Accm(types::Accm::try_from_bytes(data)?)),
    36u16 => |data| Ok(RandomVector(types::RandomVector::try_from_bytes(data)?)),
    37u16 => |data| Ok(PrivateGroupId(types::PrivateGroupId::try_from_bytes(data)?)),
    38u16 => |data| Ok(RxConnectSpeed(types::RxConnectSpeed::try_from_bytes(data)?)),
    39u16 => |data| Ok(SequencingRequired)
};

const N_HEADER_OCTETS: usize = 6;
const N_FLAG_LENGTH_VENDOR_OCTETS: usize = 4;
const ATTRIBUTE_TYPE_SIZE: usize = 2;

impl AVP {
    pub fn reveal(mut self, secret: &[u8], random_vector: &[u8]) -> ResultStr<Self> {
        match self {
            Self::Hidden(ref mut input) => {
                // The first 4 octets have been peeled off and we are guaranteed to have at least 6
                assert!(input.len() >= ATTRIBUTE_TYPE_SIZE);
                let attribute_type = unsafe { read_u16_be_unchecked(input) };

                const CHUNK_SIZE: usize = 16;
                let chunk_data = &mut input[ATTRIBUTE_TYPE_SIZE..];
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

                AVP_CODES.get(&attribute_type).map_or_else(
                    || Err("Unknown hidden AVP encountered"),
                    |constructor| constructor(chunk_data),
                )
            }
            _ => Ok(self),
        }
    }

    pub fn try_from_bytes_greedy(input: &[u8]) -> Vec<ResultStr<Self>> {
        let mut avp_start_offset = 0;
        let mut result = Vec::new();
        while avp_start_offset < input.len() {
            // Note: Subsequent unsafe code depends on this check
            if input.len() < avp_start_offset + N_HEADER_OCTETS {
                result.push(Err("Incomplete AVP header encountered"));
                break;
            }

            let (flags, length, vendor_id) =
                Self::read_flags_length_vendor(&input[avp_start_offset..]);

            let decode_start_offset = avp_start_offset + N_FLAG_LENGTH_VENDOR_OCTETS;
            let avp_end_offset = avp_start_offset + length as usize;

            let avp = if vendor_id != 0 {
                Err("AVP with unsupported vendor ID encountered")
            } else if avp_end_offset > input.len() {
                Err("AVP with invalid length field encountered")
            } else if flags.is_hidden() {
                // Hidden AVP
                let hidden_data = input[decode_start_offset..avp_end_offset].to_owned();
                Ok(Self::Hidden(hidden_data))
            } else {
                // Regular AVP
                Self::decode(&input[decode_start_offset..avp_end_offset])
            };
            result.push(avp);

            avp_start_offset = avp_end_offset;
        }

        result
    }

    fn read_flags_length_vendor(input: &[u8]) -> (Flags, u16, u16) {
        assert!(input.len() >= N_FLAG_LENGTH_VENDOR_OCTETS);

        // Flags and length share the first 2 octets
        let flags = Flags::from(input[0]);
        let msb = (input[0] >> 6) as u16;
        let lsb = input[1] as u16;
        let length = msb << 8 | lsb;

        // The second 2 octets are the Vendor ID
        let vendor_id = unsafe { read_u16_be_unchecked(&input[2..]) };

        (flags, length, vendor_id)
    }

    fn decode(input: &[u8]) -> ResultStr<Self> {
        // The first 4 octets have been peeled off and we are guaranteed to have at least 6
        assert!(input.len() >= ATTRIBUTE_TYPE_SIZE);
        let attribute_type = unsafe { read_u16_be_unchecked(input) };

        AVP_CODES.get(&attribute_type).map_or_else(
            || Err("Unknown AVP encountered"),
            |constructor| constructor(&input[ATTRIBUTE_TYPE_SIZE..]),
        )
    }
}
