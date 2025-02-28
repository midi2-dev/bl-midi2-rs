use ux::*;

pub trait BitOps {
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self;
    fn crumb(&self, index: usize) -> u2;
    fn set_crumb(&mut self, index: usize, v: u2) -> &mut Self;
    fn nibble(&self, index: usize) -> u4;
    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self;
    fn septet(&self, index: usize) -> u7;
    fn set_septet(&mut self, index: usize, v: u7) -> &mut Self;
    fn octet(&self, index: usize) -> u8;
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self;
    fn word(&self, index: usize) -> u16;
    fn set_word(&mut self, index: usize, v: u16) -> &mut Self;
}

impl BitOps for u32 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 32);
        (self >> (31 - index)) & 0b1 != 0
    }

    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 32);
        let v = u32::from(v);
        let shift = 31 - index;
        *self &= !(0b1 << shift);
        *self |= v << shift;
        self
    }

    fn crumb(&self, index: usize) -> u2 {
        assert!(index < 16);
        ((self >> (30 - index * 2)) & 0b11).try_into().unwrap()
    }

    fn set_crumb(&mut self, index: usize, v: u2) -> &mut Self {
        assert!(index < 16);
        let shift = 30 - index * 2;
        *self &= !(0b11 << shift);
        *self |= u32::from(v) << shift;
        self
    }

    fn nibble(&self, index: usize) -> u4 {
        assert!(index < 8);
        ((self >> (28 - index * 4)) & 0xF).try_into().unwrap()
    }

    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self {
        assert!(index < 8);
        let shift = 28 - index * 4;
        *self &= !(0xF << shift);
        *self |= u32::from(v) << shift;
        self
    }

    fn septet(&self, index: usize) -> u7 {
        assert!(index < 4);
        ((self >> (24 - index * 8)) & 0x7F).try_into().unwrap()
    }

    fn set_septet(&mut self, index: usize, v: u7) -> &mut Self {
        assert!(index < 4);
        let shift = 24 - index * 8;
        *self &= !(0x7F << shift);
        *self |= u32::from(v) << shift;
        self
    }

    fn octet(&self, index: usize) -> u8 {
        assert!(index < 4);
        ((self >> (24 - index * 8)) & 0xFF).try_into().unwrap()
    }

    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index < 4);
        let shift = 24 - index * 8;
        *self &= !(0xFF << shift);
        *self |= (v as u32) << shift;
        self
    }

    fn word(&self, index: usize) -> u16 {
        assert!(index < 2);
        ((self >> (16 - index * 16)) & 0xFFFF).try_into().unwrap()
    }

    fn set_word(&mut self, index: usize, v: u16) -> &mut Self {
        assert!(index < 2);
        let shift = 16 - index * 16;
        *self &= !(0xFFFF << shift);
        *self |= (v as u32) << shift;
        self
    }
}

impl BitOps for u7 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 7);
        (*self >> (6 - index)) & u7::new(0b1) != u7::new(0)
    }
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 7);
        let v = if v { u7::new(1) } else { u7::new(0) };
        let shift = 6 - index;
        *self &= !(u7::new(1) << shift);
        *self |= v << shift;
        self
    }
    fn crumb(&self, _index: usize) -> u2 {
        todo!()
    }
    fn set_crumb(&mut self, _index: usize, _v: u2) -> &mut Self {
        todo!()
    }
    fn nibble(&self, _index: usize) -> u4 {
        todo!()
    }
    fn set_nibble(&mut self, _index: usize, _v: u4) -> &mut Self {
        todo!()
    }
    fn septet(&self, _index: usize) -> u7 {
        panic!()
    }
    fn set_septet(&mut self, _index: usize, _v: u7) -> &mut Self {
        panic!()
    }
    fn octet(&self, _index: usize) -> u8 {
        panic!()
    }
    fn set_octet(&mut self, _index: usize, _v: u8) -> &mut Self {
        panic!()
    }
    fn word(&self, _index: usize) -> u16 {
        panic!()
    }
    fn set_word(&mut self, _index: usize, _v: u16) -> &mut Self {
        panic!()
    }
}

impl BitOps for u8 {
    fn bit(&self, index: usize) -> bool {
        assert!(index < 8);
        (self >> (7 - index)) & 0b1 != 0
    }
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 8);
        let v = u8::from(v);
        let shift = 7 - index;
        *self &= !(0b1 << shift);
        *self |= v << shift;
        self
    }
    fn crumb(&self, _index: usize) -> u2 {
        todo!()
    }
    fn set_crumb(&mut self, _index: usize, _v: u2) -> &mut Self {
        todo!()
    }
    fn nibble(&self, index: usize) -> u4 {
        assert!(index < 2);
        ((self >> (4 - index * 4)) & 0xF).try_into().unwrap()
    }
    fn set_nibble(&mut self, index: usize, v: u4) -> &mut Self {
        assert!(index < 2);
        let shift = 4 - index * 4;
        *self &= !(0xF << shift);
        *self |= u8::from(v) << shift;
        self
    }
    fn septet(&self, _index: usize) -> u7 {
        u7::new(self & 0x7F)
    }
    fn set_septet(&mut self, index: usize, v: u7) -> &mut Self {
        assert!(index == 0);
        *self &= !0x7F;
        *self |= u8::from(v);
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
        assert_eq!(0b1000_0000_u8.set_bit(0, false), &0x0,);
        assert_eq!(0x0_u8.set_bit(0, true), &0b1000_0000,);
        assert_eq!(0x0_u8.set_bit(4, true), &0b0000_1000,);
    }

    #[test]
    fn bit_u7() {
        let p = u7::new(0b1000010);
        assert!(p.bit(0));
        assert!(!p.bit(1));
        assert!(!p.bit(2));
        assert!(!p.bit(3));
        assert!(!p.bit(4));
        assert!(p.bit(5));
        assert!(!p.bit(6));
    }

    #[test]
    fn set_bit_u7() {
        assert_eq!(u7::new(0b1000000).set_bit(0, false), &u7::new(0x0));
        assert_eq!(u7::new(0x0).set_bit(0, true), &u7::new(0b1000000));
        assert_eq!(u7::new(0x0).set_bit(4, true), &u7::new(0b0000100));
    }

    #[test]
    fn nibble() {
        let p = 0xAB_u8;
        assert_eq!(p.nibble(0), u4::new(0xA));
        assert_eq!(p.nibble(1), u4::new(0xB));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x1_u8.set_nibble(1, u4::new(6)), &0x6,);
        assert_eq!(0x0_u8.set_nibble(0, u4::new(0xB)), &0xB0,);
        assert_eq!(0x0_u8.set_nibble(1, u4::new(0xB)), &0x0B,);
    }

    #[test]
    fn octet() {
        let p = 0xFC_u8;
        assert_eq!(p.octet(0), 0xFC);
    }

    #[test]
    fn set_octet() {
        assert_eq!(0x1_u8.set_octet(0, 0x6), &0x6);
        assert_eq!(0x0_u8.set_octet(0, 0xBE), &0xBE);
    }

    #[test]
    fn septet() {
        assert_eq!(0b0111_1111_u8.septet(0), u7::new(0b0111_1111_u8));
        assert_eq!(0b1111_1111_u8.septet(0), u7::new(0b0111_1111_u8));
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
    fn crumb() {
        let p = 0b1101_1001_0000_0000_0000_0000_0011_1001_u32;
        assert!(p.crumb(0) == u2::new(0b11));
        assert!(p.crumb(1) == u2::new(0b01));
        assert!(p.crumb(2) == u2::new(0b10));
        assert!(p.crumb(3) == u2::new(0b01));
        assert!(p.crumb(13) == u2::new(0b11));
        assert!(p.crumb(14) == u2::new(0b10));
        assert!(p.crumb(15) == u2::new(0b01));
    }

    #[test]
    fn set_crumb() {
        assert_eq!(
            0x0_u32.set_crumb(0, u2::new(0b11)),
            &0b1100_0000_0000_0000_0000_0000_0000_0000_u32,
        );
        assert_eq!(
            0x0_u32.set_crumb(4, u2::new(0b10)),
            &0b0000_0000_1000_0000_0000_0000_0000_0000_u32,
        );
        assert_eq!(
            0x0_u32.set_crumb(15, u2::new(0b01)),
            &0b0000_0000_0000_0000_0000_0000_0000_0001_u32,
        );
    }

    #[test]
    fn nibble() {
        let p = 0x2468_ACE0_u32;
        assert_eq!(p.nibble(0), u4::new(0x2));
        assert_eq!(p.nibble(3), u4::new(0x8));
        assert_eq!(p.nibble(5), u4::new(0xC));
        assert_eq!(p.nibble(7), u4::new(0x0));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(0x5A21_C612_u32.set_nibble(3, u4::new(6)), &0x5A26_C612,);
        assert_eq!(0x0_u32.set_nibble(0, u4::new(0xB)), &0xB000_0000,);
        assert_eq!(0x0_u32.set_nibble(5, u4::new(0xB)), &0x0000_0B00,);
        assert_eq!(0x0_u32.set_nibble(7, u4::new(0x4)), &0x0000_0004,);
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
