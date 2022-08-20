use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersion {
    pub version: u8,
    pub revision: u8,
}

impl ProtocolVersion {
    const ATTRIBUTE_TYPE: u16 = 2;
    const LENGTH: usize = 2;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete ProtocolVersion AVP encountered");
        }

        let version = unsafe { reader.read_u8_unchecked() };
        let revision = unsafe { reader.read_u8_unchecked() };
        Ok(Self { version, revision })
    }
}

impl QueryableAVP for ProtocolVersion {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ProtocolVersion {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&[self.version, self.revision]);
    }
}
