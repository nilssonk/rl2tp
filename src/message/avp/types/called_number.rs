use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};
use core::borrow::Borrow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalledNumber {
    pub value: String,
}

impl CalledNumber {
    const ATTRIBUTE_TYPE: u16 = 21;

    #[inline]
    pub fn try_read<T: Borrow<[u8]>>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CalledNumber AVP encountered");
        }

        let data = reader.bytes(reader.len())?;
        let value = std::str::from_utf8(data.borrow())
            .map_err(|_| "Parsing CalledNumber AVP value failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl From<String> for CalledNumber {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CalledNumber> for String {
    fn from(value: CalledNumber) -> Self {
        value.value
    }
}

impl QueryableAVP for CalledNumber {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for CalledNumber {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_bytes(self.value.as_bytes());
    }
}
