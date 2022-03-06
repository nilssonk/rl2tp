use crate::common::{Reader, ResultStr};

#[derive(Clone, Debug, PartialEq)]
pub struct CallErrors {
    pub crc_errors: u32,
    pub framing_errors: u32,
    pub hardware_overruns: u32,
    pub buffer_overruns: u32,
    pub timeout_errors: u32,
    pub alignment_errors: u32,
}

impl CallErrors {
    pub fn try_read<'a>(mut reader: Box<dyn Reader<'a> + 'a>) -> ResultStr<Self> {
        if reader.len() < 26 {
            return Err("Incomplete CallErrors AVP encountered");
        }

        // Skip reserved
        reader.skip_bytes(2);

        let crc_errors = unsafe { reader.read_u32_be_unchecked() };
        let framing_errors = unsafe { reader.read_u32_be_unchecked() };
        let hardware_overruns = unsafe { reader.read_u32_be_unchecked() };
        let buffer_overruns = unsafe { reader.read_u32_be_unchecked() };
        let timeout_errors = unsafe { reader.read_u32_be_unchecked() };
        let alignment_errors = unsafe { reader.read_u32_be_unchecked() };

        Ok(Self {
            crc_errors,
            framing_errors,
            hardware_overruns,
            buffer_overruns,
            timeout_errors,
            alignment_errors,
        })
    }
}
