use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SequencingRequired {}

impl SequencingRequired {
    const ATTRIBUTE_TYPE: u16 = 39;
}

impl QueryableAVP for SequencingRequired {
    fn get_length(&self) -> usize {
        0
    }
}

impl WritableAVP for SequencingRequired {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
    }
}
