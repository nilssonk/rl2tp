use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FramingType {
    data: u32,
}

impl FramingType {
    const LENGTH: u16 = 4;

    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete FramingType AVP encountered");
        }

        let data = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self::from_raw(data))
    }

    pub fn is_analog_request(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_request(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}

impl QueryableAVP for FramingType {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for FramingType {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
