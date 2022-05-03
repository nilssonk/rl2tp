use crate::avp::header::Header;
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct CalledNumber {
    pub value: String,
}

impl CalledNumber {
    const ATTRIBUTE_TYPE: u16 = 21;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete CalledNumber AVP encountered");
        }

        let value = std::str::from_utf8(reader.peek_bytes(reader.len())?)
            .map_err(|_| "Parsing CalledNumber advisory message failed")?
            .to_owned();

        Ok(Self { value })
    }
}

impl QueryableAVP for CalledNumber {
    fn get_length(&self) -> u16 {
        assert!(self.value.len() <= (u16::MAX - Header::LENGTH) as usize);

        Header::LENGTH + self.value.len() as u16
    }
}

impl WritableAVP for CalledNumber {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        assert!(self.value.len() <= (u16::MAX - Header::LENGTH) as usize);

        let header = Header::with_payload_length_and_attribute_type(
            self.value.len() as u16,
            Self::ATTRIBUTE_TYPE,
        );
        header.write(writer);

        writer.write_bytes_unchecked(self.value.as_bytes());
    }
}
