use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareRevision {
    pub value: u16,
}

impl FirmwareRevision {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete FirmwareRevision AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for FirmwareRevision {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl QueryableAVP for FirmwareRevision {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
