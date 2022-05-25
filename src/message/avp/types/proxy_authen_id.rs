use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProxyAuthenId {
    pub value: u8,
}

impl ProxyAuthenId {
    const ATTRIBUTE_TYPE: u16 = 32;
    const LENGTH: usize = 2;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH {
            return Err("Incomplete ProxyAuthenId AVP encountered");
        }

        // Reserved
        reader.skip_bytes(1);

        let value = unsafe { reader.read_u8_unchecked() };
        Ok(Self { value })
    }
}

impl QueryableAVP for ProxyAuthenId {
    fn get_length(&self) -> usize {
        Self::LENGTH
    }
}

impl WritableAVP for ProxyAuthenId {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        writer.write_u16_be_unchecked(Self::ATTRIBUTE_TYPE);
        writer.write_bytes_unchecked(&[0x00, self.value]);
    }
}
