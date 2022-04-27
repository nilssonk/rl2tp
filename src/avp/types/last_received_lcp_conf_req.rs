use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct LastReceivedLcpConfReq {
    pub data: Vec<u8>,
}

impl LastReceivedLcpConfReq {
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete LastReceivedLcpConfReq AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for LastReceivedLcpConfReq {
    fn get_length(&self) -> u16 {
        self.data.len() as u16
    }
}

impl WritableAVP for LastReceivedLcpConfReq {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
