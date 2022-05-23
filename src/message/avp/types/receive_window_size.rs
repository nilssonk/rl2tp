use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReceiveWindowSize {
    pub value: u16,
}

impl ReceiveWindowSize {
    const ATTRIBUTE_TYPE: u16 = 10;
    const LENGTH: usize = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete ReceiveWindowSize AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for ReceiveWindowSize {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for ReceiveWindowSize {
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ReceiveWindowSize {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be_unchecked(self.value);
    }
}
