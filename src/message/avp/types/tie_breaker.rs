use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TieBreaker {
    pub value: u64,
}

impl TieBreaker {
    const ATTRIBUTE_TYPE: u16 = 5;
    const LENGTH: usize = 8;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        let value = unsafe { reader.read_u64_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u64> for TieBreaker {
    #[inline]
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<TieBreaker> for u64 {
    #[inline]
    fn from(value: TieBreaker) -> Self {
        value.value
    }
}

impl QueryableAVP for TieBreaker {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for TieBreaker {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u64_be(self.value);
    }
}
