pub unsafe fn read_u16_be_unchecked(input: &[u8]) -> u16 {
    u16::from_be_bytes(input[..2].try_into().unwrap_unchecked())
}
