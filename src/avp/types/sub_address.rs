use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct SubAddress {
    pub value: String,
}

impl SubAddress {
    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete SubAddress AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing SubAddress advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for SubAddress {
    fn get_length(&self) -> usize {
        self.value.len()
    }
}

impl WritableAVP for SubAddress {
    #[inline]
    unsafe fn write(&self, _writer: &mut impl Writer) {
        unimplemented!();
    }
}
