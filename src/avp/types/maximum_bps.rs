use crate::common::{read_u32_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaximumBps {
    pub value: u32,
}

impl MaximumBps {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete MaximumBps AVP encountered");
        }

        let value = unsafe { read_u32_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u32> for MaximumBps {
    fn from(value: u32) -> Self {
        Self { value }
    }
}
