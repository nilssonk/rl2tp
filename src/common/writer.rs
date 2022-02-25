pub trait Writer {
    fn len(&self) -> usize;

    unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]);
    unsafe fn write_u8_unchecked(&mut self, value: u8);
    unsafe fn write_u16_be_unchecked(&mut self, value: u16);
    unsafe fn write_u32_be_unchecked(&mut self, value: u32);
    unsafe fn write_u64_be_unchecked(&mut self, value: u64);
}
