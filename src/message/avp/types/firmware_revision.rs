use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FirmwareRevision {
    pub value: u16,
}

impl FirmwareRevision {
    const ATTRIBUTE_TYPE: u16 = 6;
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

impl From<u16> for FirmwareRevision {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<FirmwareRevision> for u16 {
    #[inline]
    fn from(value: FirmwareRevision) -> Self {
        value.value
    }
}

impl QueryableAVP for FirmwareRevision {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for FirmwareRevision {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be(self.value);
    }
}
