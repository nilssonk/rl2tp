use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RxConnectSpeed {
    pub value: u32,
}

impl RxConnectSpeed {
    const ATTRIBUTE_TYPE: u16 = 38;
    const LENGTH: usize = 4;

    #[inline]
    pub fn try_read<T>(reader: &mut impl Reader<T>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete RxConnectSpeed AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for RxConnectSpeed {
    #[inline]
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<RxConnectSpeed> for u32 {
    #[inline]
    fn from(value: RxConnectSpeed) -> Self {
        value.value
    }
}

impl QueryableAVP for RxConnectSpeed {
    #[inline]
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for RxConnectSpeed {
    #[inline]
    fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be(Self::ATTRIBUTE_TYPE);
        writer.write_u32_be(self.value);
    }
}
