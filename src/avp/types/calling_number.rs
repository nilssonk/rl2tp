use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct CallingNumber {
    pub value: String,
}

impl CallingNumber {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
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
    fn get_length(&self) -> u16 {
        assert!(self.value.len() <= u16::MAX as usize);

        self.value.len() as u16
    }
}

impl WritableAVP for CallingNumber {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
