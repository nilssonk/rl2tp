use crate::common::ResultStr;

pub trait Reader<'a> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn subreader(&mut self, length: usize) -> Box<dyn Reader<'a> + 'a>;

    fn peek_bytes(&self, length: usize) -> ResultStr<&'a [u8]>;

    fn read_bytes(&mut self, length: usize) -> ResultStr<Vec<u8>>;

    unsafe fn read_u8_unchecked(&mut self) -> u8;
    unsafe fn read_u16_be_unchecked(&mut self) -> u16;
    unsafe fn read_u32_be_unchecked(&mut self) -> u32;
    unsafe fn read_u64_be_unchecked(&mut self) -> u64;

    fn skip_bytes(&mut self, length: usize);
}
