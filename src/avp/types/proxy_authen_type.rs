use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

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
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete ProxyAuthenType AVP encountered");
        }

        unsafe {
            reader
                .read_u16_be_unchecked()
                .try_into()
                .map_err(|_| "Unknown ProxyAuthenType encountered")
        }
    }
}

impl QueryableAVP for ProxyAuthenType {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
