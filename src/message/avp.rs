///! Types and implementations related to L2TP Control Message Attribute Value Pairs.

#[cfg(test)]
mod tests;

mod header;
use header::Header;

pub mod types;

use enum_dispatch::enum_dispatch;

use crate::common::{DecodeError, DecodeResult, Reader, SliceReader, VecWriter, Writer};
use core::borrow::Borrow;
use std::ops::DerefMut;

/// # Summary
/// An `AVP` is a representation of an L2TP Attribute Value Pair, of which one or more may be present in a `ControlMessage`.
///
/// ## Note
/// Currently only Attribute Value Pairs defined in the L2TP specification are supported,
/// although the specification itself purposefully leaves room for user-specified ones.
#[enum_dispatch]
#[derive(Clone, Debug, Eq, PartialEq)]
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
    fn get_length(&self) -> usize;
}

#[enum_dispatch(AVP)]
pub(crate) trait WritableAVP {
    fn write(&self, writer: &mut impl Writer);
}

use AVP::*;

pub(crate) fn avp_name(attribute_type: u16) -> String {
    let result = match attribute_type {
        0u16 => "MessageType",
        1u16 => "ResultCode",
        2u16 => "ProtocolVersion",
        3u16 => "FramingCapabilities",
        4u16 => "BearerCapabilities",
        5u16 => "TieBreaker",
        6u16 => "FirmwareRevision",
        7u16 => "HostName",
        8u16 => "VendorName",
        9u16 => "AssignedTunnelId",
        10u16 => "ReceiveWindowSize",
        11u16 => "Challenge",
        12u16 => "Q931CauseCode",
        13u16 => "ChallengeResponse",
        14u16 => "AssignedSessionId",
        15u16 => "CallSerialNumber",
        16u16 => "MinimumBps",
        17u16 => "MaximumBps",
        18u16 => "BearerType",
        19u16 => "FramingType",
        21u16 => "CalledNumber",
        22u16 => "CallingNumber",
        23u16 => "SubAddress",
        24u16 => "TxConnectSpeed",
        25u16 => "PhysicalChannelId",
        26u16 => "InitialReceivedLcpConfReq",
        27u16 => "LastSentLcpConfReq",
        28u16 => "LastReceivedLcpConfReq",
        29u16 => "ProxyAuthenType",
        30u16 => "ProxyAuthenName",
        31u16 => "ProxyAuthenChallenge",
        32u16 => "ProxyAuthenId",
        33u16 => "ProxyAuthenResponse",
        34u16 => "CallErrors",
        35u16 => "Accm",
        36u16 => "RandomVector",
        37u16 => "PrivateGroupId",
        38u16 => "RxConnectSpeed",
        39u16 => "SequencingRequired",
        x => return format!("{x}"),
    };

    result.to_owned()
}

fn decode_avp<T: Borrow<[u8]>>(
    attribute_type: u16,
    reader: &mut impl Reader<T>,
) -> DecodeResult<AVP> {
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
        x => Err(DecodeError::UnknownAvp(x))?,
    })
}

impl AVP {
    pub const CRYPTO_CHUNK_SIZE: usize = 16;

    const ATTRIBUTE_TYPE_SIZE: usize = 2;
    const LENGTH_BITS: u8 = 10;
    const MAX_LENGTH: u16 = (1 << Self::LENGTH_BITS) - 1;

    /// # Summary
    /// Convert this `AVP` into a `Hidden` AVP using the L2TP-protocol-specified encryption and padding algorithm.
    ///
    /// If this `AVP` is _already_ a `Hidden` AVP, then return it unaltered.
    ///
    /// # Parameters
    /// * `secret` - A shared secret.
    /// * `random_vector` - A `RandomVector` AVP to be shared with the receiver.
    /// * `length_padding` - Random bytes of a random length, used for length padding.
    /// * `alignment_padding` - Random bytes of length `CRYPTO_CHUNK_SIZE`, used for alignment padding.
    pub fn hide(
        self,
        secret: &[u8],
        random_vector: &types::RandomVector,
        length_padding: &[u8],
        alignment_padding: &[u8; Self::CRYPTO_CHUNK_SIZE],
    ) -> Self {
        match &self {
            Hidden(_) => self,
            avp => {
                let chunk_size: usize = Self::CRYPTO_CHUNK_SIZE;

                let mut writer = VecWriter::new();

                WritableAVP::write(avp, &mut writer);
                assert!(writer.len() >= Self::ATTRIBUTE_TYPE_SIZE);

                // Extract Attribute Type
                let attribute_type_octets: [u8; Self::ATTRIBUTE_TYPE_SIZE] =
                    writer.data[..Self::ATTRIBUTE_TYPE_SIZE].try_into().unwrap();

                // Get total AVP length
                let length =
                    writer.data.len() + Header::LENGTH as usize - Self::ATTRIBUTE_TYPE_SIZE;

                // Overwrite Attribute Type with AVP length
                assert!(length <= Self::MAX_LENGTH as usize);
                let length_octets = (length as u16).to_be_bytes();
                writer.write_bytes_at(&length_octets, 0);

                let mut input = writer.data;

                // Add random length padding
                input.extend_from_slice(length_padding);

                let chunk_padding_length = (chunk_size - (input.len() % chunk_size)) % chunk_size;

                // Pad input to chunk size
                input.extend_from_slice(&alignment_padding[..chunk_padding_length]);

                let n_chunks = input.len() / chunk_size;

                // The largest intermediate buffer size is the size of the final intermediate value
                let buffer_length =
                    Self::ATTRIBUTE_TYPE_SIZE + secret.len() + random_vector.value.len();
                let mut buffer = Vec::with_capacity(buffer_length);

                // The first intermediate value is MD5(Attribute type + secret + RV)
                buffer.extend_from_slice(&attribute_type_octets);
                buffer.extend_from_slice(secret);
                buffer.extend_from_slice(&random_vector.value);
                let mut intermediate = md5::compute(&buffer);
                // Encode with XOR
                for j in 0..chunk_size {
                    input[j] ^= intermediate[j];
                }

                if n_chunks > 1 {
                    // The shared secret is a prefix for all chunks except the first one, so set it once for the entire loop
                    buffer.clear();
                    buffer.extend_from_slice(secret);

                    // Loop over chunks
                    for i in 1..n_chunks {
                        let prev_chunk_start = (i - 1) * chunk_size;
                        let chunk_start = prev_chunk_start + chunk_size;

                        // Retain only the prefix which is guaranteed to be the shared secret
                        buffer.truncate(secret.len());

                        // The intermediate value for a given chunk is MD5(secret + previous chunk)
                        buffer.extend_from_slice(&input[prev_chunk_start..chunk_start]);
                        intermediate = md5::compute(&buffer);

                        // Encode with XOR
                        for j in 0..chunk_size {
                            input[chunk_start + j] ^= intermediate[j];
                        }
                    }
                }

                Hidden(types::Hidden {
                    attribute_type: u16::from_be_bytes(attribute_type_octets),
                    value: input,
                })
            }
        }
    }

    /// # Summary
    /// If this `AVP` is a `Hidden` AVP, then decrypt and depad its content, converting it to an `AVP` of the encapsulated type.
    ///
    /// If this `AVP` is _not_ a `Hidden` AVP, then return it unaltered.
    ///
    /// # Parameters
    /// * `secret` - A shared secret.
    /// * `random_vector` - A `RandomVector` AVP received from the same source as the `Hidden` AVP to be revealed.
    pub fn reveal(self, secret: &[u8], random_vector: &types::RandomVector) -> DecodeResult<Self> {
        if let Hidden(mut hidden) = self {
            let chunk_size: usize = Self::CRYPTO_CHUNK_SIZE;

            let chunk_data = &mut hidden.value;

            if chunk_data.is_empty() {
                return Err(DecodeError::EmptyHiddenAVP);
            }
            if chunk_data.len() % chunk_size != 0 {
                return Err(DecodeError::MisalignedHiddenAVP);
            }

            let n_chunks = chunk_data.len() / chunk_size;

            // The largest size is the size of the final intermediate value
            let buffer_length =
                Self::ATTRIBUTE_TYPE_SIZE + secret.len() + random_vector.value.len();
            let mut buffer = Vec::with_capacity(buffer_length);

            if n_chunks > 1 {
                // The shared secret is a prefix for all chunks except the first one, so set it once for the entire loop
                buffer.extend_from_slice(secret);

                // Loop over chunks in reverse order
                for i in (1..n_chunks).rev() {
                    let prev_chunk_start = (i - 1) * chunk_size;
                    let chunk_start = prev_chunk_start + chunk_size;

                    // Retain only the prefix which is guaranteed to be the shared secret
                    buffer.truncate(secret.len());

                    // The intermediate value for a given chunk is MD5(secret + previous chunk)
                    buffer.extend_from_slice(&chunk_data[prev_chunk_start..chunk_start]);
                    let intermediate = md5::compute(&buffer);

                    // Decode with XOR
                    for j in 0..chunk_size {
                        chunk_data[chunk_start + j] ^= intermediate[j];
                    }
                }
            }

            // The final intermediate value is MD5(Attribute type + secret + RV)
            buffer.clear();
            buffer.extend_from_slice(&hidden.attribute_type.to_be_bytes());
            buffer.extend_from_slice(secret);
            buffer.extend_from_slice(&random_vector.value);
            let intermediate = md5::compute(&buffer);

            // Decode with XOR
            for j in 0..chunk_size {
                chunk_data[j] ^= intermediate[j];
            }

            // Retreive original length, SliceReader implementation of Reader is safe
            let mut reader = SliceReader::from(chunk_data.deref_mut());
            let total_length = unsafe { reader.read_u16_be_unchecked() };
            if !(Header::LENGTH..=Self::MAX_LENGTH).contains(&total_length) {
                return Err(DecodeError::InvalidOriginalAVPLength(total_length));
            }
            let payload_length = total_length - Header::LENGTH;

            // Decode payload
            let mut payload_reader = reader.subreader(payload_length as usize);

            return decode_avp(hidden.attribute_type, &mut payload_reader);
        }

        Ok(self)
    }

    /// # Summary
    /// Try to greedily read a list of `AVP`s using a `Reader`.
    ///
    /// Reading will proceed until the `Reader` is empty or an invalid AVP header length field is encountered.
    #[inline]
    pub fn try_read_greedy<T: Borrow<[u8]>>(
        reader: &mut impl Reader<T>,
    ) -> Vec<DecodeResult<Self>> {
        let mut result = Vec::new();
        while let Some(header) = Header::try_read(reader) {
            if header.payload_length as usize > reader.len() {
                result.push(Err(DecodeError::InvalidAVPLength(header.payload_length)));
                break;
            }
            if header.vendor_id != 0 {
                result.push(Err(DecodeError::UnsupportedVendorId(header.vendor_id)));
                reader.skip_bytes(header.payload_length as usize);
                continue;
            }

            let avp = if header.flags.is_hidden() {
                // Hidden AVP
                let hidden_data = reader
                    .bytes(header.payload_length as usize)
                    .map(|x| x.borrow().to_owned())
                    .unwrap_or_default();
                Ok(Self::Hidden(types::Hidden {
                    attribute_type: header.attribute_type,
                    value: hidden_data,
                }))
            } else {
                // Regular AVP
                let mut subreader = reader.subreader(header.payload_length as usize);
                decode_avp(header.attribute_type, &mut subreader)
            };
            result.push(avp);
        }

        result
    }

    /// # Summary
    /// Get the length of this `AVP`.
    #[inline]
    pub fn get_length(&self) -> usize {
        QueryableAVP::get_length(self)
    }

    #[inline]
    fn make_flags_and_length(is_mandatory: bool, is_hidden: bool, length: usize) -> [u8; 2] {
        assert!(length <= Self::MAX_LENGTH as usize);

        let msb = ((length >> 8) & 0x3) as u8;
        let lsb = length as u8;
        let m_bit = is_mandatory as u8;
        let h_bit = (is_hidden as u8) << 1;
        let octet1 = (msb << 6) | m_bit | h_bit;
        let octet2 = lsb;
        [octet1, octet2]
    }

    /// # Summary
    /// Write an `AVP` using a `Writer`.
    #[inline]
    pub fn write(&self, writer: &mut impl Writer) {
        const VENDOR_ID: u16 = 0;
        const IS_MANDATORY: bool = true;

        // Save header position
        let start_position = writer.len();

        // Dummy octets to be overwritten
        writer.write_bytes(&[0, 0]);

        // Write rest of header
        writer.write_u16_be(VENDOR_ID);

        // Write payload
        WritableAVP::write(self, writer);

        // Get total length
        let end_position = writer.len();
        let length = end_position - start_position;

        let is_hidden = matches!(self, Hidden(_));

        let flags_and_length = Self::make_flags_and_length(IS_MANDATORY, is_hidden, length);

        // Oerwrite dummy octets
        writer.write_bytes_at(&flags_and_length, start_position);
    }
}
