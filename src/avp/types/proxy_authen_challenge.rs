use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenChallenge {
    pub data: Vec<u8>,
}

impl ProxyAuthenChallenge {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete ProxyAuthenChallenge AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}
