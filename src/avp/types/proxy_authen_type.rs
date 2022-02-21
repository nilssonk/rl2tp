use crate::common::{read_u16_be_unchecked, ResultStr};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, PartialEq)]
#[repr(u16)]
pub enum ProxyAuthenType {
    Reserved,
    TextualUserNamePasswordExchange,
    PppChap,
    PppPap,
    NoAuthentication,
    MicrosoftChapVersion1,
}

impl ProxyAuthenType {
    pub fn from(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 2 {
            return Err("Incomplete ProxyAuthenType AVP encountered");
        }

        let value = unsafe { read_u16_be_unchecked(input) };
        value
            .try_into()
            .map_err(|_| "Unknown ProxyAuthenType encountered")
    }
}
