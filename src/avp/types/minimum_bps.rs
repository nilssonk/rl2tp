use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimumBps {
    pub value: u32,
}

impl MinimumBps {
    const LENGTH: u16 = 4;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete MinimumBps AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for MinimumBps {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for MinimumBps {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for MinimumBps {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
