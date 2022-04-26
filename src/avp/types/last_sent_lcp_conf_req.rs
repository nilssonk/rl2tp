use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct LastSentLcpConfReq {
    pub value: Vec<u8>,
}

impl LastSentLcpConfReq {
    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete LastSentLcpConfReq AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for LastSentLcpConfReq {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        assert!(self.value.len() <= u16::MAX as usize);

        (self.value.len() as u16, 0)
    }
}

impl WritableAVP for LastSentLcpConfReq {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
