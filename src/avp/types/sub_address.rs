use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct SubAddress {
    pub value: String,
}

impl SubAddress {
    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete SubAddress AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing SubAddress advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for SubAddress {
    fn get_length(&self) -> u16 {
        assert!(self.value.len() <= u16::MAX as usize);

        self.value.len() as u16
    }
}

impl WritableAVP for SubAddress {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
