use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenResponse {
    pub data: Vec<u8>,
}

impl ProxyAuthenResponse {
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete ProxyAuthenResponse AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for ProxyAuthenResponse {
    fn get_length(&self) -> u16 {
        self.data.len() as u16
    }
}

impl WritableAVP for ProxyAuthenResponse {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
