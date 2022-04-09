use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProtocolVersion {
    pub version: u8,
    pub revision: u8,
}

impl ProtocolVersion {
    const LENGTH: u16 = 2;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete ProtocolVersion AVP encountered");
        }

        let version = unsafe { reader.read_u8_unchecked() };
        let revision = unsafe { reader.read_u8_unchecked() };
        Ok(Self { version, revision })
    }
}

impl QueryableAVP for ProtocolVersion {
    fn get_length(&self) -> u16 {
        Self::LENGTH
    }
}

impl WritableAVP for ProtocolVersion {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
