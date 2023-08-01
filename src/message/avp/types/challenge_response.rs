use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

const G_CHALLENGE_RESPONSE_LENGTH: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChallengeResponse {
    pub value: [u8; G_CHALLENGE_RESPONSE_LENGTH],
}

impl ChallengeResponse {
    const ATTRIBUTE_TYPE: u16 = 13;
    const LENGTH: usize = G_CHALLENGE_RESPONSE_LENGTH;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete ChallengeResponse AVP encountered");
        }

        Ok(Self {
            value: reader
                .bytes(16)?
                .borrow()
                .try_into()
                .map_err(|_| "Insufficient data")?,
        })
    }
}

impl From<[u8; G_CHALLENGE_RESPONSE_LENGTH]> for ChallengeResponse {
    fn from(value: [u8; G_CHALLENGE_RESPONSE_LENGTH]) -> Self {
        Self { value }
    }
}

impl From<ChallengeResponse> for [u8; G_CHALLENGE_RESPONSE_LENGTH] {
    fn from(value: ChallengeResponse) -> Self {
        value.value
    }
}

impl QueryableAVP for ChallengeResponse {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ChallengeResponse {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
