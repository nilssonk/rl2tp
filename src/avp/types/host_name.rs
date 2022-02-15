use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct HostName {
    pub data: Vec<u8>,
}

impl HostName {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete HostName AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}
