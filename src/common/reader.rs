/// # Summary
/// A trait representing an abstract reader.
///
/// The type parameter `T` affords generic borrowing and lifetime management e.g.
/// using the Borrow trait. It should essentially be thought of as a means of
/// getting a byte slice `&[u8]`.
pub trait Reader<T> {
    /// # Summary
    /// Indicate whether the `Reader` is empty.
    fn is_empty(&self) -> bool;

    /// # Summary
    /// Get the number of remaining bytes.
    fn len(&self) -> usize;

    /// # Summary
    /// Consume `length` bytes of data and produce a new `Reader` containing it.
    fn subreader(&mut self, length: usize) -> Self
    where
        Self: Sized;

    /// # Summary
    /// Attempt to read a generic byte slice `length` bytes long.
    fn bytes(&mut self, length: usize) -> Option<T>;

    /// # Summary
    /// Read a single u8.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn read_u8_unchecked(&mut self) -> u8;

    /// # Summary
    /// Read a native u16 from big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn read_u16_be_unchecked(&mut self) -> u16;

    /// # Summary
    /// Read a native u32 from big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn read_u32_be_unchecked(&mut self) -> u32;

    /// # Summary
    /// Read a native u64 from big endian data.
    /// # Safety
    /// This function is marked as unsafe because it offers no error handling mechanism.
    unsafe fn read_u64_be_unchecked(&mut self) -> u64;

    /// # Summary
    /// Skip `length` bytes of `Reader` data.
    fn skip_bytes(&mut self, length: usize);
}
