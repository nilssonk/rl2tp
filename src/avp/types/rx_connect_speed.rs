use crate::common::{read_u32_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RxConnectSpeed {
    pub value: u32,
}

impl RxConnectSpeed {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 4 {
            return Err("Incomplete RxConnectSpeed AVP encountered");
        }

        let value = unsafe { read_u32_be_unchecked(input) };
        Ok(Self { value })
    }
}

impl From<u32> for RxConnectSpeed {
    fn from(value: u32) -> Self {
        Self { value }
    }
}
