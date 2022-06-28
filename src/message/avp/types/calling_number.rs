use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallingNumber {
    pub value: String,
}

impl CallingNumber {
    const ATTRIBUTE_TYPE: u16 = 22;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CallingNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CallingNumber AVP value failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl From<String> for CallingNumber {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<CallingNumber> for String {
    fn from(value: CallingNumber) -> Self {
        value.value
    }
}

impl QueryableAVP for CallingNumber {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for CallingNumber {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
