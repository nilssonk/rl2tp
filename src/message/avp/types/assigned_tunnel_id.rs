use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignedTunnelId {
    pub value: u16,
}

impl AssignedTunnelId {
    const ATTRIBUTE_TYPE: u16 = 9;
    const LENGTH: usize = 2;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete AssignedTunnelId AVP encountered");
        }

        let value = unsafe { reader.read_u16_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u16> for AssignedTunnelId {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<AssignedTunnelId> for u16 {
    #[inline]
    fn from(value: AssignedTunnelId) -> Self {
        value.value
    }
}

impl QueryableAVP for AssignedTunnelId {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for AssignedTunnelId {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u16_be(self.value);
    }
}
