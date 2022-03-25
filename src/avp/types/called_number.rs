use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct CalledNumber {
    pub value: String,
}

impl CalledNumber {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CalledNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CalledNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for CalledNumber {
    fn get_length(&self) -> u16 {
        assert!(self.value.len() <= u16::MAX as usize);

        self.value.len() as u16
    }
}

impl WritableAVP for CalledNumber {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
