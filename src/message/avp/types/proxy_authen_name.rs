use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProxyAuthenName {
    pub value: Vec<u8>,
}

impl ProxyAuthenName {
    const ATTRIBUTE_TYPE: u16 = 30;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.is_empty() {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        Ok(Self {
            value: reader
                .bytes(reader.len())
                .ok_or(DecodeError::AVPReadError(Self::ATTRIBUTE_TYPE))?
                .borrow()
                .to_owned(),
        })
    }
}

impl From<Vec<u8>> for ProxyAuthenName {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl From<ProxyAuthenName> for Vec<u8> {
    fn from(value: ProxyAuthenName) -> Self {
        value.value
    }
}

impl QueryableAVP for ProxyAuthenName {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for ProxyAuthenName {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(&self.value);
    }
}
