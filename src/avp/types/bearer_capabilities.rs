use crate::common::{read_u32_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BearerCapabilities {
    data: u32,
}

impl BearerCapabilities {
    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete BearerCapabilities AVP encountered");
        }

        let data = unsafe { read_u32_be_unchecked(input) };
        Ok(Self::from_raw(data))
    }

    pub fn is_analog_access_supported(&self) -> bool {
        ((self.data >> 6) & 0x1) != 0
    }

    pub fn is_digital_access_supported(&self) -> bool {
        ((self.data >> 7) & 0x1) != 0
    }
}