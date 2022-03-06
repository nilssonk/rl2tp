use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TxConnectSpeed {
    pub value: u32,
}

impl TxConnectSpeed {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 4 {
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
