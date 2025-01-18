use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Challenge {
    pub value: Vec<u8>,
}

impl Challenge {
    const ATTRIBUTE_TYPE: u16 = 11;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.is_empty() {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        Ok(Self {
            value: reader
                .bytes(reader.len())
                .ok_or(DecodeError::AVPReadError(Self::ATTRIBUTE_TYPE))?
                .borrow()
                .to_owned(),
        })
    }
}

impl From<Vec<u8>> for Challenge {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl From<Challenge> for Vec<u8> {
    fn from(value: Challenge) -> Self {
        value.value
    }
}

impl QueryableAVP for Challenge {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for Challenge {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
