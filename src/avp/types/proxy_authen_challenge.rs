use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct ProxyAuthenChallenge {
    pub value: Vec<u8>,
}

impl ProxyAuthenChallenge {
    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete ProxyAuthenChallenge AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for ProxyAuthenChallenge {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        assert!(self.value.len() <= u16::MAX as usize);

        (self.value.len() as u16, 0)
    }
}

impl WritableAVP for ProxyAuthenChallenge {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
