#[cfg(test)]
mod tests;

mod header;
use header::Header;

pub mod types;

use enum_dispatch::enum_dispatch;

use crate::common::{Reader, ResultStr, SliceReader, Writer};

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
pub(crate) trait QueryableAVP {
    fn get_length(&self) -> u16;
}

#[enum_dispatch(AVP)]
pub(crate) trait WritableAVP {
    unsafe fn write(&self, writer: &mut dyn Writer);
}

use AVP::*;

fn decode_avp(attribute_type: u16, reader: &mut dyn Reader) -> ResultStr<AVP> {
    Ok(match attribute_type {
        0u16 => MessageType(types::MessageType::try_read(reader)?),
        1u16 => ResultCode(types::ResultCode::try_read(reader)?),
        2u16 => ProtocolVersion(types::ProtocolVersion::try_read(reader)?),
        3u16 => FramingCapabilities(types::FramingCapabilities::try_read(reader)?),
        4u16 => BearerCapabilities(types::BearerCapabilities::try_read(reader)?),
        5u16 => TieBreaker(types::TieBreaker::try_read(reader)?),
        6u16 => FirmwareRevision(types::FirmwareRevision::try_read(reader)?),
        7u16 => HostName(types::HostName::try_read(reader)?),
        8u16 => VendorName(types::VendorName::try_read(reader)?),
        9u16 => AssignedTunnelId(types::AssignedTunnelId::try_read(reader)?),
        10u16 => ReceiveWindowSize(types::ReceiveWindowSize::try_read(reader)?),
        11u16 => Challenge(types::Challenge::try_read(reader)?),
        12u16 => Q931CauseCode(types::Q931CauseCode::try_read(reader)?),
        13u16 => ChallengeResponse(types::ChallengeResponse::try_read(reader)?),
        14u16 => AssignedSessionId(types::AssignedSessionId::try_read(reader)?),
        15u16 => CallSerialNumber(types::CallSerialNumber::try_read(reader)?),
        16u16 => MinimumBps(types::MinimumBps::try_read(reader)?),
        17u16 => MaximumBps(types::MaximumBps::try_read(reader)?),
        18u16 => BearerType(types::BearerType::try_read(reader)?),
        19u16 => FramingType(types::FramingType::try_read(reader)?),
        21u16 => CalledNumber(types::CalledNumber::try_read(reader)?),
        22u16 => CallingNumber(types::CallingNumber::try_read(reader)?),
        23u16 => SubAddress(types::SubAddress::try_read(reader)?),
        24u16 => TxConnectSpeed(types::TxConnectSpeed::try_read(reader)?),
        25u16 => PhysicalChannelId(types::PhysicalChannelId::try_read(reader)?),
        26u16 => InitialReceivedLcpConfReq(types::InitialReceivedLcpConfReq::try_read(reader)?),
        27u16 => LastSentLcpConfReq(types::LastSentLcpConfReq::try_read(reader)?),
        28u16 => LastReceivedLcpConfReq(types::LastReceivedLcpConfReq::try_read(reader)?),
        29u16 => ProxyAuthenType(types::ProxyAuthenType::try_read(reader)?),
        30u16 => ProxyAuthenName(types::ProxyAuthenName::try_read(reader)?),
        31u16 => ProxyAuthenChallenge(types::ProxyAuthenChallenge::try_read(reader)?),
        32u16 => ProxyAuthenId(types::ProxyAuthenId::try_read(reader)?),
        33u16 => ProxyAuthenResponse(types::ProxyAuthenResponse::try_read(reader)?),
        34u16 => CallErrors(types::CallErrors::try_read(reader)?),
        35u16 => Accm(types::Accm::try_read(reader)?),
        36u16 => RandomVector(types::RandomVector::try_read(reader)?),
        37u16 => PrivateGroupId(types::PrivateGroupId::try_read(reader)?),
        38u16 => RxConnectSpeed(types::RxConnectSpeed::try_read(reader)?),
        39u16 => SequencingRequired(types::SequencingRequired::default()),
        _ => Err("Unknown AVP encountered")?,
    })
}

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

                let mut reader = SliceReader::from(chunk_data);
                decode_avp(attribute_type, &mut reader)
            }
            _ => Ok(self),
        }
    }

    pub fn try_read_greedy(reader: &mut dyn Reader) -> Vec<ResultStr<Self>> {
        let mut result = Vec::new();
        while let Some(header) = Header::try_read(reader) {
            if header.payload_length as usize > reader.len() {
                result.push(Err("AVP with invalid length field encountered"));
                break;
            }
            if header.vendor_id != 0 {
                result.push(Err("AVP with unsupported vendor ID encountered"));
                reader.skip_bytes(header.payload_length as usize);
                continue;
            }

            let avp = if header.flags.is_hidden() {
                // Hidden AVP
                let hidden_data = reader
                    .read_bytes(header.payload_length as usize)
                    .unwrap_or_default();
                Ok(Self::Hidden(types::Hidden { data: hidden_data }))
            } else {
                // Regular AVP
                let mut subreader = reader.subreader(header.payload_length as usize);
                decode_avp(header.attribute_type, subreader.as_mut())
            };
            result.push(avp);
        }

        result
    }

    #[inline]
    pub fn get_length(&self) -> u16 {
        QueryableAVP::get_length(self)
    }

    /// # Summary
    /// Write an `AVP` using a `Writer`.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    #[inline]
    pub unsafe fn write(&self, writer: &mut dyn Writer) {
        WritableAVP::write(self, writer);
    }
}
