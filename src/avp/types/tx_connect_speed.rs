use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TxConnectSpeed {
    pub value: u32,
}

impl TxConnectSpeed {
    const LENGTH: u16 = 4;

    pub fn try_read(reader: &mut dyn Reader) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete TxConnectSpeed AVP encountered");
        }

        let value = unsafe { reader.read_u32_be_unchecked() };
        Ok(Self { value })
    }
}

impl From<u32> for TxConnectSpeed {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl QueryableAVP for TxConnectSpeed {
    fn get_length_attribute_type(&self) -> (u16, u16) {
        (Self::LENGTH, 0)
    }
}

impl WritableAVP for TxConnectSpeed {
    unsafe fn write(&self, _writer: &mut dyn Writer) {
        unimplemented!();
    }
}
