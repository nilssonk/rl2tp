use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, PartialEq)]
pub struct Hidden {
    pub value: Vec<u8>,
}

impl QueryableAVP for Hidden {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        assert!(self.value.len() <= u16::MAX as usize);

        (self.value.len() as u16, 0)
    }
}

impl WritableAVP for Hidden {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
