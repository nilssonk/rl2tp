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

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
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
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for FramingType {
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
