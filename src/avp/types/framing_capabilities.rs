use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FramingCapabilities {
    data: u32,
}

impl FramingCapabilities {
    const LENGTH: u16 = 4;

    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete FramingCapabilities AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self::from_raw(data))
    }

    pub fn is_async_framing_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_sync_framing_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for FramingCapabilities {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for FramingCapabilities {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
