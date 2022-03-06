use crate::common::{Reader, ResultStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Accm {
    pub send_accm: [u8; 4],
    pub receive_accm: [u8; 4],
}

impl Accm {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 10 {
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
