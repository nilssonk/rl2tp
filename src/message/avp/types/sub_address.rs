use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubAddress {
    pub value: String,
}

impl SubAddress {
    const ATTRIBUTE_TYPE: u16 = 23;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete SubAddress AVP encountered");
        }

        let data = reader.bytes(reader.len())?;
        let value = std::str::from_utf8(data.borrow())
            .map_err(|_| "Parsing SubAddress advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl From<String> for SubAddress {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<SubAddress> for String {
    fn from(value: SubAddress) -> Self {
        value.value
    }
}

impl QueryableAVP for SubAddress {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for SubAddress {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
