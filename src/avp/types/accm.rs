use crate::avp::header::{Flags, Header};
use crate::avp::{QueryableAVP, WritableAVP};
use crate::common::{Reader, ResultStr, Writer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Accm {
    pub send_accm: [u8; 4],
    pub receive_accm: [u8; 4],
}

impl Accm {
    const ATTRIBUTE_TYPE: u16 = 35;
    const LENGTH: u16 = 10;

    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < Self::LENGTH as usize {
            return Err("Incomplete Accm AVP encountered");
        }

        // Skip reserved
        reader.skip_bytes(2);

        let send_accm = unsafe { reader.read_bytes(4)?.try_into().unwrap_unchecked() };
        let receive_accm = unsafe { reader.read_bytes(4)?.try_into().unwrap_unchecked() };

        Ok(Self {
            send_accm,
            receive_accm,
        })
    }
}

impl QueryableAVP for Accm {
    fn get_length(&self) -> u16 {
        Header::LENGTH + Self::LENGTH
    }
}

impl WritableAVP for Accm {
    unsafe fn write(&self, writer: &mut dyn Writer) {
        let header = Header {
            flags: Flags::new(true, false),
            payload_length: Self::LENGTH,
            vendor_id: 0,
            attribute_type: Self::ATTRIBUTE_TYPE,
        };
        header.write(writer);

        // Reserved
        writer.write_bytes_unchecked(&[0x00, 0x00]);

        writer.write_bytes_unchecked(&self.send_accm);
        writer.write_bytes_unchecked(&self.receive_accm);
    }
}
