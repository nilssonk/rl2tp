use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VendorName {
    pub value: String,
}

impl VendorName {
    const ATTRIBUTE_TYPE: u16 = 8;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete VendorName AVP encountered");
        }

        let data = reader.bytes(reader.len())?;
        let value = std::str::from_utf8(data.borrow())
            .map_err(|_| "Parsing VendorName AVP value failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl From<String> for VendorName {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<VendorName> for String {
    fn from(value: VendorName) -> Self {
        value.value
    }
}

impl QueryableAVP for VendorName {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for VendorName {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
