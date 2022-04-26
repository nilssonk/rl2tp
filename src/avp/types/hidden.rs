use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, PartialEq)]
pub struct Hidden {
    pub data: Vec<u8>,
}

impl QueryableAVP for Hidden {
    fn get_length(&self) -> u16 {
        assert!(self.data.len() <= u16::MAX as usize);

        self.data.len() as u16
    }
}

impl WritableAVP for Hidden {
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
