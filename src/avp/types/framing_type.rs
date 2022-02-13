use crate::common::{read_u32_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FramingType {
    data: u32,
}

impl FramingType {
    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete FramingType AVP encountered");
        }

        let data = unsafe { read_u32_be_unchecked(input) };
        Ok(Self::from_raw(data))
    }

    pub fn is_analog_request(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_request(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}
