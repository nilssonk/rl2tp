use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct VendorName {
    pub data: Vec<u8>,
}

impl VendorName {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete VendorName AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}
