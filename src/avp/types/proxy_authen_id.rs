use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProxyAuthenId {
    pub value: u8,
}

impl ProxyAuthenId {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete ProxyAuthenId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(value.into())
    }
}

impl From<u16> for ProxyAuthenId {
    fn from(value: u16) -> Self {
        let masked = (value & 0xff) as u8;
        Self { value: masked }
    }
}

impl QueryableAVP for ProxyAuthenId {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
