use crate::avp::QueryableAVP;
use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProtocolVersion {
    pub version: u8,
    pub revision: u8,
}

impl ProtocolVersion {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 2 {
            return Err("Incomplete ProtocolVersion AVP encountered");
        }

        let version = unsafe { reader.read_u8_unchecked() };
        let revision = unsafe { reader.read_u8_unchecked() };
        Ok(Self { version, revision })
    }
}

impl QueryableAVP for ProtocolVersion {
    fn get_length(&self) -> u16 {
        unimplemented!();
    }
}
