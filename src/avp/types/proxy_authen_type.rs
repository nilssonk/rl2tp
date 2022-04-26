use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

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
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for ProxyAuthenType {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
