use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallSerialNumber {
    pub value: u32,
}

impl CallSerialNumber {
    const ATTRIBUTE_TYPE: u16 = 15;
    const LENGTH: u16 = 4;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete CallSerialNumber AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for CallSerialNumber {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for CallSerialNumber {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, Self::ATTRIBUTE_TYPE)
    }
}

impl WritableAVP for CallSerialNumber {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        writer.write_u32_be_unchecked(self.value);
    }
}
