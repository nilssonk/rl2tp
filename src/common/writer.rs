pub trait Writer {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    /// # Summary
    /// Write a slice of u8.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_bytes_unchecked(&mut self, bytes: &[u8]);

    /// # Summary
    /// Write a slice of u8 at a given offset.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_bytes_unchecked_at(&mut self, bytes: &[u8], offset: usize);

    /// # Summary
    /// Write an u8.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_u8_unchecked(&mut self, value: u8);

    /// # Summary
    /// Write a native u16 as big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_u16_be_unchecked(&mut self, value: u16);

    /// # Summary
    /// Write a native u32 as big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_u32_be_unchecked(&mut self, value: u32);

    /// # Summary
    /// Write a native u64 as big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn write_u64_be_unchecked(&mut self, value: u64);
}
