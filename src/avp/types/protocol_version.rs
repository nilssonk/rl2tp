use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProtocolVersion {
    pub version: u8,
    pub revision: u8,
}

impl ProtocolVersion {
    const LENGTH: usize = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete ProtocolVersion AVP encountered");
        }

        let version = unsafe { reader.read_u8_unchecked() };
        let revision = unsafe { reader.read_u8_unchecked() };
        Ok(Self { version, revision })
    }
}

impl QueryableAVP for ProtocolVersion {
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ProtocolVersion {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
