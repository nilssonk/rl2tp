use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{DecodeError, DecodeResult, Reader, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TxConnectSpeed {
    pub value: u32,
}

impl TxConnectSpeed {
    const ATTRIBUTE_TYPE: u16 = 24;
    const LENGTH: usize = 4;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> DecodeResult<Self> {
        if reader.len() < Self::LENGTH {
            return Err(DecodeError::IncompleteAVP(Self::ATTRIBUTE_TYPE));
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for TxConnectSpeed {
    #[inline]
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<TxConnectSpeed> for u32 {
    #[inline]
    fn from(value: TxConnectSpeed) -> Self {
        value.value
    }
}

impl QueryableAVP for TxConnectSpeed {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for TxConnectSpeed {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be(self.value);
    }
}
