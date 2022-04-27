use crate::avp::Header;
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct Challenge {
    pub value: Vec<u8>,
}

impl Challenge {
    const ATTRIBUTE_TYPE: u16 = 11;

    pub fn try_read<'a, 'b>(reader: &'b mut impl Reader<'a>) -> ResultStr<Self> {
        if reader.is_empty() {
            return Err("Incomplete Challenge AVP encountered");
        }

        Ok(Self {
            value: reader.read_bytes(reader.len())?,
        })
    }
}

impl QueryableAVP for Challenge {
    fn get_length(&self) -> u16 {
        assert!(self.value.len() <= (u16::MAX - Header::LENGTH) as usize);

        Header::LENGTH + self.value.len() as u16
    }
}

impl WritableAVP for Challenge {
    #[inline]
    unsafe fn write(&self, writer: &mut impl Writer) {
        assert!(self.value.len() <= (u16::MAX - Header::LENGTH) as usize);

        let header = Header::with_payload_length_and_attribute_type(
            self.value.len() as u16,
            Self::ATTRIBUTE_TYPE,
        );
        header.write(writer);

        writer.write_bytes_unchecked(&self.value);
    }
}
