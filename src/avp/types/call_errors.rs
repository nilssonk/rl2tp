use crate::common::{read_u32_be_unchecked, ResultStr};

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
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 26 {
            return Err("Incomplete CallErrors AVP encountered");
        }

        // Skip reserved
        let mut offset = 2;

        let crc_errors = unsafe { read_u32_be_unchecked(&input[offset..]) };
        offset += 4;
        let framing_errors = unsafe { read_u32_be_unchecked(&input[offset..]) };
        offset += 4;
        let hardware_overruns = unsafe { read_u32_be_unchecked(&input[offset..]) };
        offset += 4;
        let buffer_overruns = unsafe { read_u32_be_unchecked(&input[offset..]) };
        offset += 4;
        let timeout_errors = unsafe { read_u32_be_unchecked(&input[offset..]) };
        offset += 4;
        let alignment_errors = unsafe { read_u32_be_unchecked(&input[offset..]) };

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
