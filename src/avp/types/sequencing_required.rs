use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SequencingRequired {}

impl QueryableAVP for SequencingRequired {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (0, 0)
    }
}

impl WritableAVP for SequencingRequired {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
