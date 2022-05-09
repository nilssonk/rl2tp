use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, PartialEq)]
pub struct Hidden {
    pub data: Vec<u8>,
}

impl QueryableAVP for Hidden {
    fn get_length(&self) -> usize {
        self.data.len()
    }
}

impl WritableAVP for Hidden {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
