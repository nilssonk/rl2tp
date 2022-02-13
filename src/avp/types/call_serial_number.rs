use crate::common::{read_u32_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallSerialNumber {
    pub value: u32,
}

impl CallSerialNumber {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete CallSerialNumber AVP encountered");
        }

        let value = unsafe { read_u32_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u32> for CallSerialNumber {
    fn from(value: u32) -> Self {
        Self { value }
    }
}
