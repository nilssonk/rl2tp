use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReceiveWindowSize {
    pub value: u16,
}

impl ReceiveWindowSize {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete ReceiveWindowSize AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u16> for ReceiveWindowSize {
    fn from(value: u16) -> Self {
        Self { value }
    }
}
