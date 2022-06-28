use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateGroupId {
    pub value: Vec<u8>,
}

impl PrivateGroupId {
    const ATTRIBUTE_TYPE: u16 = 37;

    #[inline]
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete PrivateGroupId AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl From<Vec<u8>> for PrivateGroupId {
    fn from(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl From<PrivateGroupId> for Vec<u8> {
    fn from(value: PrivateGroupId) -> Self {
        value.value
    }
}

impl QueryableAVP for PrivateGroupId {
    #[inline]
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for PrivateGroupId {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&self.value);
    }
}
