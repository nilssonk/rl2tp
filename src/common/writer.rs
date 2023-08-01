/// # Summary
/// A trait representing an abstract writer.
pub trait Writer {
    /// # Summary
    /// Indicate whether the `Writer` is empty.
    fn is_empty(&self) -> bool;

    /// # Summary
    /// Get the number of written bytes.
    fn len(&self) -> usize;

    /// # Summary
    /// Write byte slice.
    fn write_bytes(&mut self, bytes: &[u8]);

    /// # Summary
    /// Write a byte slice at a given offset.
    fn write_bytes_at(&mut self, bytes: &[u8], offset: usize);

    /// # Summary
    /// Write a single u8.
    fn write_u8(&mut self, value: u8);

    /// # Summary
    /// Write a native u16 as big endian data.
    fn write_u16_be(&mut self, value: u16);

    /// # Summary
    /// Write a native u32 as big endian data.
    fn write_u32_be(&mut self, value: u32);

    /// # Summary
    /// Write a native u64 as big endian data.
    fn write_u64_be(&mut self, value: u64);
}
