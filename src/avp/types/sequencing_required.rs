use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SequencingRequired {}

impl QueryableAVP for SequencingRequired {
    fn get_length(&self) -> u16 {
        0
    }
}

impl WritableAVP for SequencingRequired {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
