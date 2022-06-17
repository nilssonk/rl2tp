use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::Writer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hidden {
    pub attribute_type: u16,
    pub value: Vec<u8>,
}

impl QueryableAVP for Hidden {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for Hidden {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(self.attribute_type);
        writer.write_bytes_unchecked(&self.value);
    }
}
