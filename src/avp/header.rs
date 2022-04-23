#[cfg(test)]
mod tests;

mod flags;
pub use flags::Flags;

use crate::common::{Reader, Writer};

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub flags: Flags,
    pub payload_length: u16,
    pub vendor_id: u16,
    pub attribute_type: u16,
}

impl Header {
    pub const LENGTH: u16 = 6;

    pub fn with_payload_length_and_attribute_type(
        payload_length: u16,
        attribute_type: u16,
    ) -> Self {
        Self {
            flags: Flags::new(true, false),
            payload_length,
            vendor_id: 0,
            attribute_type,
        }
    }

    /// # Summary
    /// Attempt to read a `Header` using a `Reader`. Fails if there isn't enough data.
    pub fn try_read(reader: &mut dyn Reader) -> Option<Self> {
        // Note: Subsequent unsafe code depends on this check
        if reader.len() < Self::LENGTH as usize {
            return None;
        }

        // Flags and length share the first 2 octets
        let octet1 = unsafe { reader.read_u8_unchecked() };
        let octet2 = unsafe { reader.read_u8_unchecked() };

        let flags = Flags::from(octet1);

        let msb = (octet1 >> 6) as u16;
        let lsb = octet2 as u16;
        let length = msb << 8 | lsb;

        // The second 2 octets are the Vendor ID
        let vendor_id = unsafe { reader.read_u16_be_unchecked() };

        // The final 2 octets are the Attribute Type
        let attribute_type = unsafe { reader.read_u16_be_unchecked() };

        let payload_length = length - Self::LENGTH;

        Some(Header {
            flags,
            payload_length,
            vendor_id,
            attribute_type,
        })
    }

    /// # Summary
    /// Write a `Header` using a mutable `Writer`.
    /// # Safety
    /// This function is marked as unsafe because the `Writer` trait offers no error handling mechanism.
    pub unsafe fn write(&self, writer: &mut dyn Writer) {
        let wire_length = Self::LENGTH + self.payload_length;

        // Length split
        let msb = ((wire_length >> 8) & 0x3) as u8;
        let lsb = wire_length as u8;

        let m_bit = self.flags.is_mandatory() as u8;
        let h_bit = (self.flags.is_hidden() as u8) << 1;
        let octet1 = m_bit | h_bit | (msb << 6);
        let octet2 = lsb;

        writer.write_u8_unchecked(octet1);
        writer.write_u8_unchecked(octet2);
        writer.write_u16_be_unchecked(self.vendor_id);
        writer.write_u16_be_unchecked(self.attribute_type);
    }
}
