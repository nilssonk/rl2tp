use super::*;

#[test]
fn from() {
    // All bits
    {
        let f = Flags::from(0xff);
        assert_eq!(f, Flags { data: 0x3f });
    }
    // No bits
    {
        let f = Flags::from(0x00);
        assert_eq!(f, Flags { data: 0x0000 });
    }
}

#[test]
fn get_bit() {
    // All bits
    {
        let f = Flags::from(0xff);
        for i in 0..6 {
            let value = f.get_bit(i);
            assert_eq!(value, true);
        }
    }
    // No bits
    {
        let f = Flags::from(0x00);
        for i in 0..6 {
            let value = f.get_bit(i);
            assert_eq!(value, false);
        }
    }
    // First bit
    {
        let f = Flags::from(0x01);
        for i in 0..6 {
            let value = f.get_bit(i);
            if i == 0 {
                assert_eq!(value, true);
            } else {
                assert_eq!(value, false);
            }
        }
    }
    // Last bit
    {
        let f = Flags::from(0x20);
        for i in 0..6 {
            let value = f.get_bit(i);
            if i == 5 {
                assert_eq!(value, true);
            } else {
                assert_eq!(value, false);
            }
        }
    }
}

#[test]
fn is_mandatory() {
    // All bits
    {
        let f = Flags::from(0xff);
        assert_eq!(f.is_mandatory(), true);
    }
    // All but relevant bit
    {
        let f = Flags::from(0xfe);
        assert_eq!(f.is_mandatory(), false);
    }
    // No bits
    {
        let f = Flags::from(0x00);
        assert_eq!(f.is_mandatory(), false);
    }
    // None but relevant bit
    {
        let f = Flags::from(0x01);
        assert_eq!(f.is_mandatory(), true);
    }
}

#[test]
fn is_hidden() {
    // All bits
    {
        let f = Flags::from(0xff);
        assert_eq!(f.is_hidden(), true);
    }
    // All but relevant bit
    {
        let f = Flags::from(0xfd);
        assert_eq!(f.is_hidden(), false);
    }
    // No bits
    {
        let f = Flags::from(0x00);
        assert_eq!(f.is_hidden(), false);
    }
    // None but relevant bit
    {
        let f = Flags::from(0x02);
        assert_eq!(f.is_hidden(), true);
    }
}

#[test]
fn reserved_bits_ok() {
    // All bits
    {
        let f = Flags::from(0xff);
        assert_eq!(f.reserved_bits_ok(), false);
    }
    // All but relevant bits
    {
        let f = Flags::from(0x03);
        assert_eq!(f.reserved_bits_ok(), true);
    }
    // No bits
    {
        let f = Flags::from(0x00);
        assert_eq!(f.reserved_bits_ok(), true);
    }
    // None but relevant bits
    {
        let f = Flags::from(0xfc);
        assert_eq!(f.reserved_bits_ok(), false);
    }
}
