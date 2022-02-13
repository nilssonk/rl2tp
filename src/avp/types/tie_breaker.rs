use crate::common::{read_u64_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TieBreaker {
    pub value: u64,
}

impl TieBreaker {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 8 {
            return Err("Incomplete TieBreaker AVP encountered");
        }

        let value = unsafe { read_u64_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u64> for TieBreaker {
    fn from(value: u64) -> Self {
        Self { value }
    }
}
