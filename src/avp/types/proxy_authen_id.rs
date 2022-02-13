use crate::common::{read_u16_be_unchecked, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProxyAuthenId {
    pub value: u8,
}

impl ProxyAuthenId {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete ProxyAuthenId AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        Ok(value.into())
    }
}

impl From<u16> for ProxyAuthenId {
    fn from(value: u16) -> Self {
        let masked = (value & 0xff) as u8;
        Self { value: masked }
    }
}
