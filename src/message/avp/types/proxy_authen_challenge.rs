use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProxyAuthenChallenge {
    pub value: Vec<u8>,
}

impl ProxyAuthenChallenge {
    const ATTRIBUTE_TYPE: u16 = 31;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete ProxyAuthenChallenge AVP encountered");
        }

        Ok(Self {
            value: reader.bytes(reader.len())?.borrow().to_owned(),
        })
    }
}

impl From<Vec<u8>> for ProxyAuthenChallenge {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl From<ProxyAuthenChallenge> for Vec<u8> {
    fn from(value: ProxyAuthenChallenge) -> Self {
        value.value
    }
}

impl QueryableAVP for ProxyAuthenChallenge {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for ProxyAuthenChallenge {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
