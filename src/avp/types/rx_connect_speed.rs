use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RxConnectSpeed {
    pub value: u32,
}

impl RxConnectSpeed {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 4 {
            return Err("Incomplete RxConnectSpeed AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for RxConnectSpeed {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for RxConnectSpeed {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
