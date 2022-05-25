use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalledNumber {
    pub value: String,
}

impl CalledNumber {
    const ATTRIBUTE_TYPE: u16 = 21;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CalledNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CalledNumber AVP value failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for CalledNumber {
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for CalledNumber {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
