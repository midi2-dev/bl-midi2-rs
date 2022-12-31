pub trait BitOps {
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self;
    fn nibble(&self, index: usize) -> ux::u4;
    fn set_nibble(&mut self, index: usize, v: ux::u4) -> &mut Self;
    fn octet(&self, index: usize) -> u8;
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self;
    fn word(&self, index: usize) -> u16;
    fn set_word(&mut self, index: usize, v: u16) -> &mut Self;
}

impl BitOps for u32 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 32);
        self >> (31 - index) & 0b1 != 0
    }

    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 32);
        let v: u32 = match v {
            true => 1,
            false => 0,
        };
        *self |= v << (31 - index);
        self
    }

    fn nibble(&self, index: usize) -> ux::u4 {
        assert!(index < 8);
        ((self >> (28 - index * 4)) & 0xF).try_into().unwrap()
    }

    fn set_nibble(&mut self, index: usize, v: ux::u4) -> &mut Self {
        assert!(index < 8);
        *self |= u32::from(v) << (28 - index * 4);
        self
    }

    fn octet(&self, index: usize) -> u8 {
        assert!(index < 4);
        (self >> (24 - index * 8) & 0xFF).try_into().unwrap()
    }

    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index < 4);
        *self |= (v as u32) << (24 - index * 8);
        self
    }

    fn word(&self, index: usize) -> u16 {
        assert!(index < 2);
        (self >> (16 - index * 16) & 0xFFFF).try_into().unwrap()
    }

    fn set_word(&mut self, index: usize, v: u16) -> &mut Self {
        assert!(index < 2);
        *self |= (v as u32) << (16 - index * 16);
        self
    }
}

impl BitOps for u8 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 8);
        self >> (7 - index) & 0b1 != 0
    }
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 8);
        #[allow(clippy::bool_to_int_with_if)] // clippy bug?
        let v: u8 = if v { 1 } else { 0 };
        *self |= v << (7 - index);
        self
    }
    fn nibble(&self, index: usize) -> ux::u4 {
        assert!(index < 2);
        ((self >> (4 - index * 4)) & 0xF).try_into().unwrap()
    }
    fn set_nibble(&mut self, index: usize, v: ux::u4) -> &mut Self {
        assert!(index < 2);
        *self |= u8::from(v) << (4 - index * 4);
        self
    }
    fn octet(&self, _index: usize) -> u8 {
        *self
    }
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index == 0);
        *self = v;
        self
    }
    fn word(&self, _index: usize) -> u16 {
        panic!()
    }
    fn set_word(&mut self, _index: usize, _v: u16) -> &mut Self {
        panic!()
    }
    }

#[cfg(test)]
mod tests_u8 {
    use super::*;

    #[test]
    fn bit() {
        let p = 0b1000_0010_u8;
        assert!(p.bit(0));
        assert!(!p.bit(1));
        assert!(p.bit(6));
        assert!(!p.bit(7));
    }

    #[test]
    fn set_bit() {
        assert_eq!(0x0_u8.set_bit(0, true), &0b1000_0000,);
        assert_eq!(0x0_u8.set_bit(4, true), &0b0000_1000,);
    }

    #[test]
    fn nibble() {
        let p = 0xAB_u8;
        assert_eq!(p.nibble(0), ux::u4::new(0xA));
        assert_eq!(p.nibble(1), ux::u4::new(0xB));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x0_u8.set_nibble(0, ux::u4::new(0xB)), &0xB0,);
        assert_eq!(0x0_u8.set_nibble(1, ux::u4::new(0xB)), &0x0B,);
    }

    #[test]
    fn octet() {
        let p = 0xFC_u8;
        assert_eq!(p.octet(0), 0xFC);
    }

    #[test]
    fn set_octet() {
        assert_eq!(0x0_u8.set_octet(0, 0xBE), &0xBE);
    }
}

#[cfg(test)]
mod tests_u32 {
    use super::*;

    #[test]
    fn bit() {
        let p = 0b1000_0000_0000_0000_0000_0000_0000_0010_u32;
        assert!(p.bit(0));
        assert!(!p.bit(1));
        assert!(p.bit(30));
        assert!(!p.bit(31));
    }

    #[test]
    fn set_bit() {
        assert_eq!(
            0x0_u32.set_bit(0, true),
            &0b1000_0000_0000_0000_0000_0000_0000_0000_u32,
        );
        assert_eq!(
            0x0_u32.set_bit(10, true),
            &0b0000_0000_0010_0000_0000_0000_0000_0000,
        );
    }

    #[test]
    fn nibble() {
        let p = 0x2468_ACE0_u32;
        assert_eq!(p.nibble(0), ux::u4::new(0x2));
        assert_eq!(p.nibble(3), ux::u4::new(0x8));
        assert_eq!(p.nibble(5), ux::u4::new(0xC));
        assert_eq!(p.nibble(7), ux::u4::new(0x0));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x0_u32.set_nibble(0, ux::u4::new(0xB)), &0xB000_0000,);
        assert_eq!(0x0_u32.set_nibble(5, ux::u4::new(0xB)), &0x0000_0B00,);
        assert_eq!(0x0_u32.set_nibble(7, ux::u4::new(0x4)), &0x0000_0004,);
    }

    #[test]
    fn octet() {
        let p = 0x0123_4567_u32;
        assert_eq!(p.octet(0), 0x01);
        assert_eq!(p.octet(1), 0x23);
        assert_eq!(p.octet(2), 0x45);
        assert_eq!(p.octet(3), 0x67);
    }

    #[test]
    fn set_octet() {
        assert_eq!(0x0_u32.set_octet(0, 0xBE), &0xBE00_0000,);
        assert_eq!(0x0_u32.set_octet(2, 0xBE), &0x0000_BE00,);
        assert_eq!(0x0_u32.set_octet(3, 0xBE), &0x0000_00BE,);
    }

    #[test]
    fn word() {
        let p = 0x0123_4567_u32;
        assert_eq!(p.word(0), 0x0123);
        assert_eq!(p.word(1), 0x4567);
    }

    #[test]
    fn set_word() {
        assert_eq!(0x0_u32.set_word(0, 0x0ABE), &0x0ABE_0000,);
        assert_eq!(0x0_u32.set_word(1, 0x0ABE), &0x0000_0ABE,);
    }
}
