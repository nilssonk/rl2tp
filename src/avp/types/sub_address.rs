use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct SubAddress {
    pub value: String,
}

impl SubAddress {
    pub fn try_read<'a>(reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
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
        unimplemented!();
    }
}
