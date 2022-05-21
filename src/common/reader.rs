use crate::common::ResultStr;

pub trait Reader<'a>: Send {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn subreader(&mut self, length: usize) -> Self
    where
        Self: Sized;

    fn peek_bytes(&self, length: usize) -> ResultStr<&'a [u8]>;

    fn read_bytes(&mut self, length: usize) -> ResultStr<Vec<u8>>;

    /// # Summary
    /// Read an u8
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

    fn skip_bytes(&mut self, length: usize);
}
