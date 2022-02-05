pub unsafe fn read_u16_be_unchecked(input: &[u8]) -> u16 {
    u16::from_be_bytes(input[..2].try_into().unwrap_unchecked())
}

pub unsafe fn read_u32_be_unchecked(input: &[u8]) -> u32 {
    u32::from_be_bytes(input[..4].try_into().unwrap_unchecked())
}
