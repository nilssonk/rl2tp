use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignedSessionId {
    pub value: u16,
}

impl AssignedSessionId {
    const ATTRIBUTE_TYPE: u16 = 14;
    const LENGTH: usize = 2;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedSessionId {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<AssignedSessionId> for u16 {
    #[inline]
    fn from(value: AssignedSessionId) -> Self {
        value.value
    }
}

impl QueryableAVP for AssignedSessionId {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for AssignedSessionId {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be(self.value);
    }
}
