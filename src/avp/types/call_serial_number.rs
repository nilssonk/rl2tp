use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallSerialNumber {
    pub value: u32,
}

impl CallSerialNumber {
    const LENGTH: u16 = 4;

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
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
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for CallSerialNumber {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
