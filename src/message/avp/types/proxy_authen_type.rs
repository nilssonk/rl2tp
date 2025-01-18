use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
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
    const ATTRIBUTE_TYPE: u16 = 29;
    const LENGTH: usize = 2;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        unsafe {
            reader
                .read_u16_be_unchecked()
                .try_into()
                .map_err(|_| DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE))
        }
    }
}

impl QueryableAVP for ProxyAuthenType {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ProxyAuthenType {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be((*self).into());
    }
}
