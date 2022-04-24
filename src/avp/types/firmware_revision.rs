use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareRevision {
    pub value: u16,
}

impl FirmwareRevision {
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
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
        Self::LENGTH
    }
}

impl WritableAVP for FirmwareRevision {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
