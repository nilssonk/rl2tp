use crate::common::ResultStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Accm {
    pub send_accm: [u8; 4],
    pub receive_accm: [u8; 4],
}

impl Accm {
    pub fn try_from_bytes(input: &[u8]) -> ResultStr<Self> {
        if input.len() < 10 {
            return Err("Incomplete Accm AVP encountered");
        }

        // Skip reserved
        let mut offset = 2;

        let send_accm = unsafe { input[offset..offset + 4].try_into().unwrap_unchecked() };
        offset += 4;
        let receive_accm = unsafe { input[offset..offset + 4].try_into().unwrap_unchecked() };

        Ok(Self {
            send_accm,
            receive_accm,
        })
    }
}
