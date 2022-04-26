use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReceiveWindowSize {
    pub value: u16,
}

impl ReceiveWindowSize {
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for ReceiveWindowSize {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
