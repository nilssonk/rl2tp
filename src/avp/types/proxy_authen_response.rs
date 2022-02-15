use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenResponse {
    pub data: Vec<u8>,
}

impl ProxyAuthenResponse {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete ProxyAuthenResponse AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}
