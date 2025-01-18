use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FramingCapabilities {
    data: u32,
}

impl FramingCapabilities {
    const ATTRIBUTE_TYPE: u16 = 3;
    const LENGTH: usize = 4;

    #[inline]
    pub fn new(async_framing_supported: bool, sync_framing_supported: bool) -> Self {
        let async_bit = (async_framing_supported as u32) << 6;
        let sync_bit = (sync_framing_supported as u32) << 7;
        Self {
            data: async_bit | sync_bit,
        }
    }

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { data })
    }

    #[inline]
    pub fn is_async_framing_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    #[inline]
    pub fn is_sync_framing_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for FramingCapabilities {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for FramingCapabilities {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be(self.data);
    }
}
