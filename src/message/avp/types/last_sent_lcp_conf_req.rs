use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LastSentLcpConfReq {
    pub value: Vec<u8>,
}

impl LastSentLcpConfReq {
    const ATTRIBUTE_TYPE: u16 = 27;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete LastSentLcpConfReq AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl From<Vec<u8>> for LastSentLcpConfReq {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl From<LastSentLcpConfReq> for Vec<u8> {
    fn from(value: LastSentLcpConfReq) -> Self {
        value.value
    }
}

impl QueryableAVP for LastSentLcpConfReq {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for LastSentLcpConfReq {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&self.value);
    }
}
