#[derive(
    Default,
    PartialEq,
)]
pub struct Packet {
    pub data: [u32; 4],
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            data: [0, 0, 0, 0],
        }
    }

    pub fn group(&self) -> u8 {
        self.nibble(1)
    }

    pub fn set_group(self, g: u8) -> Self {
        self.set_nibble(1, g)
    }

    pub fn nibble(&self, index: usize) -> u8 {
        assert!(index < 32);
        ((self.data[index / 8] >> (28 - (index % 8) * 4)) & 0xF)
            .try_into()
            .unwrap()
    }
    
    pub fn set_nibble(mut self, index: usize, v: u8) -> Self {
        assert!(v <= 0xF);
        assert!(index < 32);
        self.data[index / 8] |= (v as u32) << (28 - (index % 8) * 4);
        self
    }

    pub fn octet(&self, index: usize) -> u8 {
        assert!(index < 16);
        ((self.data[index / 4] >> (24 - (index % 4) * 8)) & 0xFF)
            .try_into()
            .unwrap()
    }

    pub fn set_octet(mut self, index: usize, v: u8) -> Self {
        assert!(index < 16);
        self.data[index / 4] |= (v as u32) << (24 - (index % 4) * 8);
        self
    }

    pub fn word(&self, index: usize) -> u16 {
        assert!(index < 8);
        ((self.data[index / 2] >> (16 - (index % 2) * 16)) & 0xFFFF)
            .try_into()
            .unwrap()
    }

    pub fn set_word(mut self, index: usize, v: u16) -> Self {
        assert!(index < 8);
        self.data[index / 2] |= (v as u32) << (16 - (index % 2) * 16);
        self
    }
}

impl std::fmt::Debug for Packet { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Packet {{ data: [ {:#010X}, {:#010X}, {:#010X}, {:#010X} ] }}",  
            self.data[0],
            self.data[1],
            self.data[2],
            self.data[3],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_packet() {
        let packet = Packet::new();
        assert_eq!(packet, Packet{ data: [0, 0, 0, 0] });
    }

    #[test]
    fn group_reported() {
        let data_group_pairings: Vec<([u32; 4], u8)> = vec![
            ([0x0000_0000, 0x0, 0x0, 0x0], 0),
            ([0x0100_0000, 0x0, 0x0, 0x0], 1),
            ([0x0200_0000, 0x0, 0x0, 0x0], 2),
            ([0x0300_0000, 0x0, 0x0, 0x0], 3),
            ([0x0400_0000, 0x0, 0x0, 0x0], 4),
            ([0x0500_0000, 0x0, 0x0, 0x0], 5),
            ([0x0600_0000, 0x0, 0x0, 0x0], 6),
            ([0x0700_0000, 0x0, 0x0, 0x0], 7),
            ([0x0800_0000, 0x0, 0x0, 0x0], 8),
            ([0x0900_0000, 0x0, 0x0, 0x0], 9),
            ([0x0A00_0000, 0x0, 0x0, 0x0], 10),
            ([0x0B00_0000, 0x0, 0x0, 0x0], 11),
            ([0x0C00_0000, 0x0, 0x0, 0x0], 12),
            ([0x0D00_0000, 0x0, 0x0, 0x0], 13),
            ([0x0E00_0000, 0x0, 0x0, 0x0], 14),
            ([0x0F00_0000, 0x0, 0x0, 0x0], 15),
        ];
        for (d, g) in data_group_pairings {
            assert_eq!(Packet{data: d}.group(), g);
        }
    }

    #[test]
    fn set_group() {
        assert_eq!(
            Packet{ data: [ 0x0, 0x0, 0x0, 0x0 ] }.set_group(2),
            Packet{ data: [ 0x0200_0000, 0x0, 0x0, 0x0 ] },
        );
    }

    #[test]
    fn nibble() {
        let p = Packet {
            data: [
                0x0123_4567,
                0x89AB_CDEF,
                0x0123_4567,
                0x89AB_CDEF,
            ]
        };
        assert_eq!(p.nibble(0), 0);
        assert_eq!(p.nibble(3), 3);
        assert_eq!(p.nibble(16), 0);
        assert_eq!(p.nibble(19), 3);
    }

    #[test]
    fn set_nibble() {
        assert_eq!(
            Packet::new().set_nibble(0, 0xB),
            Packet { data: [0xB000_0000, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_nibble(5, 0xB),
            Packet { data: [0x0000_0B00, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_nibble(10, 0xB),
            Packet { data: [0x0, 0x00B0_0000, 0x0, 0x0] },
        );
    }

    #[test]
    fn octet() {
        let p = Packet {
            data: [
                0x0123_4567,
                0x89AB_CDEF,
                0x0123_4567,
                0x89AB_CDEF,
            ]
        };
        assert_eq!(p.octet(0), 0x01);
        assert_eq!(p.octet(3), 0x67);
        assert_eq!(p.octet(8), 0x01);
        assert_eq!(p.octet(13), 0xAB);
    }

    #[test]
    fn set_octet() {
        assert_eq!(
            Packet::new().set_octet(0, 0xBE),
            Packet { data: [0xBE00_0000, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_octet(2, 0xBE),
            Packet { data: [0x0000_BE00, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_octet(5, 0xBE),
            Packet { data: [0x0, 0x00BE_0000, 0x0, 0x0] },
        );
    }

    #[test]
    fn word() {
        let p = Packet {
            data: [
                0x0123_4567,
                0x89AB_CDEF,
                0x0123_4567,
                0x89AB_CDEF,
            ]
        };
        assert_eq!(p.word(0), 0x0123);
        assert_eq!(p.word(1), 0x4567);
        assert_eq!(p.word(6), 0x89AB);
    }

    #[test]
    fn set_word() {
        assert_eq!(
            Packet::new().set_word(0, 0x0ABE),
            Packet { data: [0x0ABE_0000, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_word(1, 0x0ABE),
            Packet { data: [0x0000_0ABE, 0x0, 0x0, 0x0] },
        );
        assert_eq!(
            Packet::new().set_word(3, 0x0ABE),
            Packet { data: [0x0, 0x0000_0ABE, 0x0, 0x0] },
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            format!("{:?}", Packet{ data: [
                0x0123_4567,
                0x89AB_CDEF,
                0x0123_4567,
                0x89AB_CDEF,
            ]}),
            "Packet { data: [ 0x01234567, 0x89ABCDEF, 0x01234567, 0x89ABCDEF ] }"
        );
    }
}
