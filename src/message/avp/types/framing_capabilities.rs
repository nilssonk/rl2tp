use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FramingCapabilities {
    data: u32,
}

impl FramingCapabilities {
    const ATTRIBUTE_TYPE: u16 = 3;
    const LENGTH: usize = 4;

    pub fn new(async_framing_supported: bool, sync_framing_supported: bool) -> Self {
        let async_bit = (async_framing_supported as u32) << 6;
        let sync_bit = (sync_framing_supported as u32) << 7;
        Self {
            data: async_bit | sync_bit,
        }
    }

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete FramingCapabilities AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { data })
    }

    pub fn is_async_framing_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_sync_framing_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for FramingCapabilities {
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for FramingCapabilities {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be_unchecked(self.data);
    }
}
