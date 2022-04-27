use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct HostName {
    pub data: Vec<u8>,
}

impl HostName {
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete HostName AVP encountered");
        }

        Ok(Self {
            data: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for HostName {
    fn get_length(&self) -> u16 {
        assert!(self.data.len() <= u16::MAX as usize);

        self.data.len() as u16
    }
}

impl WritableAVP for HostName {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
