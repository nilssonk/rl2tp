use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct PrivateGroupId {
    pub data: Vec<u8>,
}

impl PrivateGroupId {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete PrivateGroupId AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for PrivateGroupId {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
