#[derive(Clone, Default, PartialEq)]
pub struct Packet([u32; 4]);

impl Packet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_data(d: &[u32]) -> Self {
        assert!(d.len() <= 4);
        let mut p: Packet = Default::default();
        p.0[0..d.len()].copy_from_slice(d);
        p
    }
}

pub trait PacketMethods : {
    fn set_group(&mut self, g: ux::u4) -> &mut Self;
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self;
    fn nibble(&self, index: usize) -> ux::u4;
    fn set_nibble(&mut self, index: usize, v: ux::u4) -> &mut Self;
    fn octet(&self, index: usize) -> u8;
    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self;
    fn octets<'a>(&self, begin: usize, data: &'a mut [u8]) -> &'a [u8];
    fn set_octets(&mut self, begin: usize, data: &[u8]) -> &mut Self;
    fn word(&self, index: usize) -> u16;
    fn set_word(&mut self, index: usize, v: u16) -> &mut Self;
}

impl PacketMethods for Packet {

    fn set_group(&mut self, g: ux::u4) -> &mut Self {
        self.set_nibble(1, g)
    }

    fn bit(&self, index: usize) -> bool {
        assert!(index < 128);
        (self[index / 32] >> (31 - (index % 32))) & 0b1 != 0
    }

    fn set_bit(&mut self, index: usize, v: bool) -> &mut Self {
        assert!(index < 128);
        let v: u32 = match v {
            true => 1,
            false => 0,
        };
        self[index / 32] |= v << (31 - (index % 32));
        self
    }

    fn nibble(&self, index: usize) -> ux::u4 {
        assert!(index < 32);
        ((self[index / 8] >> (28 - (index % 8) * 4)) & 0xF)
            .try_into()
            .unwrap()
    }

    fn set_nibble(&mut self, index: usize, v: ux::u4) -> &mut Self {
        assert!(index < 32);
        self[index / 8] |= u32::from(v) << (28 - (index % 8) * 4);
        self
    }

    fn octet(&self, index: usize) -> u8 {
        assert!(index < 16);
        ((self[index / 4] >> (24 - (index % 4) * 8)) & 0xFF)
            .try_into()
            .unwrap()
    }

    fn set_octet(&mut self, index: usize, v: u8) -> &mut Self {
        assert!(index < 16);
        self[index / 4] |= (v as u32) << (24 - (index % 4) * 8);
        self
    }

    fn octets<'a>(&self, begin: usize, data: &'a mut [u8]) -> &'a [u8] {
        assert!(begin + data.len() < 17);
        for i in 0..data.len() {
            data[i] = self.octet((i + begin).into()); 
        }
        data
    }

    fn set_octets(&mut self, begin: usize, data: &[u8]) -> &mut Self {
        assert!(begin + data.len() < 17);
        for o in data.iter().enumerate() {
            self.set_octet(o.0 + begin, *o.1);
        }
        self
    }

    fn word(&self, index: usize) -> u16 {
        assert!(index < 8);
        ((self[index / 2] >> (16 - (index % 2) * 16)) & 0xFFFF)
            .try_into()
            .unwrap()
    }

    fn set_word(&mut self, index: usize, v: u16) -> &mut Self {
        assert!(index < 8);
        self[index / 2] |= (v as u32) << (16 - (index % 2) * 16);
        self
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Packet ([ {:#010X}, {:#010X}, {:#010X}, {:#010X} ])",
            self.0[0], self.0[1], self.0[2], self.0[3],
        )
    }
}

impl core::ops::Deref for Packet {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_packet() {
        assert_eq!(<Packet as Default>::default(), Packet([0, 0, 0, 0]));
    }

    #[test]
    #[should_panic]
    fn from_data_too_long() {
        let _ = Packet::from_data(&[0x0, 0x0, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn bit() {
        let p = Packet([
            0b1000_0000_0000_0000_0000_0000_0000_0010,
            0b0111_1111_1111_1111_1111_1111_1111_0111,
            0b0000_0010_0000_0000_0000_0000_0000_0000,
            0b1111_0111_1111_1111_1111_1111_1111_1111,
        ]);
        assert_eq!(p.bit(0), true);
        assert_eq!(p.bit(30), true);
        assert_eq!(p.bit(32), false);
        assert_eq!(p.bit(60), false);
        assert_eq!(p.bit(70), true);
        assert_eq!(p.bit(100), false);
    }

    #[test]
    fn set_bit() {
        assert_eq!(
            <Packet as Default>::default().set_bit(0, true),
            &Packet([0b1000_0000_0000_0000_0000_0000_0000_0000, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_bit(10, true),
            &Packet([0b0000_0000_0010_0000_0000_0000_0000_0000, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_bit(74, true),
            &Packet([0x0, 0x0, 0b0000_0000_0010_0000_0000_0000_0000_0000, 0x0]),
        );
    }

    #[test]
    fn nibble() {
        let p = Packet([0x0123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF]);
        assert_eq!(p.nibble(0), ux::u4::new(0));
        assert_eq!(p.nibble(3), ux::u4::new(3));
        assert_eq!(p.nibble(16), ux::u4::new(0));
        assert_eq!(p.nibble(19), ux::u4::new(3));
    }

    #[test]
    fn set_nibble() {
        assert_eq!(
            <Packet as Default>::default().set_nibble(0, ux::u4::new(0xB)),
            &Packet([0xB000_0000, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_nibble(5, ux::u4::new(0xB)),
            &Packet([0x0000_0B00, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_nibble(10, ux::u4::new(0xB)),
            &Packet([0x0, 0x00B0_0000, 0x0, 0x0]),
        );
    }

    #[test]
    fn octet() {
        let p = Packet([0x0123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF]);
        assert_eq!(p.octet(0), 0x01);
        assert_eq!(p.octet(3), 0x67);
        assert_eq!(p.octet(8), 0x01);
        assert_eq!(p.octet(13), 0xAB);
    }

    #[test]
    fn set_octet() {
        assert_eq!(
            <Packet as Default>::default().set_octet(0, 0xBE),
            &Packet([0xBE00_0000, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_octet(2, 0xBE),
            &Packet([0x0000_BE00, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_octet(5, 0xBE),
            &Packet([0x0, 0x00BE_0000, 0x0, 0x0]),
        );
    }

    #[test]
    fn octets() {
        {
            let mut data: [u8; 4] = Default::default();
            assert_eq!(
                &vec![0x12, 0x34, 0x56, 0x78],
                Packet([0x0012_3456, 0x7800_0000, 0x0, 0x0]).octets(1, &mut data),
            );
        }
        {
            let mut data: [u8; 16] = Default::default();
            assert_eq!(
                Packet([0x0012_3456, 0x7890_ABCD, 0xEF12_3456, 0x7890_ABCD]).octets(0, &mut data),
                &vec![
                    0x00, 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
                    0xAB, 0xCD,
                ],
            );
        }
        {
            assert_eq!(
                Packet([0x0, 0x0, 0x0, 0x0]).octets(0, &mut []),
                &vec![],
            );
        }
    }

    #[test]
    fn set_octets() {
        assert_eq!(
            <Packet as Default>::default().set_octets(2, &vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
            &Packet([0x0000_FFFF, 0xFFFF_FF00, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_octets(0, &Vec::new()),
            &Packet([0x0, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_octets(0, &[0xFF].repeat(16)),
            &Packet([0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF]),
        );
    }

    #[test]
    fn word() {
        let p = Packet([0x0123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF]);
        assert_eq!(p.word(0), 0x0123);
        assert_eq!(p.word(1), 0x4567);
        assert_eq!(p.word(6), 0x89AB);
    }

    #[test]
    fn set_word() {
        assert_eq!(
            <Packet as Default>::default().set_word(0, 0x0ABE),
            &Packet([0x0ABE_0000, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_word(1, 0x0ABE),
            &Packet([0x0000_0ABE, 0x0, 0x0, 0x0]),
        );
        assert_eq!(
            <Packet as Default>::default().set_word(3, 0x0ABE),
            &Packet([0x0, 0x0000_0ABE, 0x0, 0x0]),
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            format!(
                "{:?}",
                Packet([0x0123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF,]),
            ),
            "Packet ([ 0x01234567, 0x89ABCDEF, 0x01234567, 0x89ABCDEF ])",
        );
    }
}
