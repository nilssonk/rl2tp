use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct CallingNumber {
    pub value: String,
}

impl CallingNumber {
    const ATTRIBUTE_TYPE: u16 = 22;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CallingNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CallingNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for CallingNumber {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        assert!(self.value.len() <= u16::MAX as usize);

        (self.value.len() as u16, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for CallingNumber {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
