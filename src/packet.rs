use crate::group_id::GroupID;

#[derive(Debug, PartialEq)]
pub struct Packet {
    data: [u32; 4],
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Utility,
    System,
    Midi1ChannelVoice,
    SystemExclusiveData,
    Midi2ChannelVoice,
    Data,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            data: [0, 0, 0, 0],
        }
    }

    pub fn message_type(&self) -> Type {
        match self.data[0] >> 28 & 0xF {
            0x0 => Type::Utility,
            0x1 => Type::System,
            0x2 => Type::Midi1ChannelVoice,
            0x3 => Type::SystemExclusiveData,
            0x4 => Type::Midi2ChannelVoice,
            0x5 => Type::Data,
            _ => panic!(),
        }
    }

    pub fn group(&self) -> GroupID {
        match self.data[0] >> 24 & 0xF {
            0x0 => GroupID::G1,
            0x1 => GroupID::G2,
            0x2 => GroupID::G3,
            0x3 => GroupID::G4,
            0x4 => GroupID::G5,
            0x5 => GroupID::G6,
            0x6 => GroupID::G7,
            0x7 => GroupID::G8,
            0x8 => GroupID::G9,
            0x9 => GroupID::G10,
            0xA => GroupID::G11,
            0xB => GroupID::G12,
            0xC => GroupID::G13,
            0xD => GroupID::G14,
            0xE => GroupID::G15,
            0xF => GroupID::G16,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_packet() {
        let packet = Packet::new();
        assert_eq!(packet, Packet{ data: [0, 0, 0, 0] });
    }

    #[test]
    fn message_type_reported() {
        let data_type_pairings: Vec<([u32; 4], Type)> = vec![
            ([0x0000_0000, 0x0, 0x0, 0x0], Type::Utility),
            ([0x1000_0000, 0x0, 0x0, 0x0], Type::System),
            ([0x2000_0000, 0x0, 0x0, 0x0], Type::Midi1ChannelVoice),
            ([0x3000_0000, 0x0, 0x0, 0x0], Type::SystemExclusiveData),
            ([0x4000_0000, 0x0, 0x0, 0x0], Type::Midi2ChannelVoice),
            ([0x5000_0000, 0x0, 0x0, 0x0], Type::Data),
        ];
        for (d, t) in data_type_pairings {
            assert_eq!(Packet{data: d}.message_type(), t);
        }
    }

    #[test]
    fn group_reported() {
        let data_group_pairings: Vec<([u32; 4], GroupID)> = vec![
            ([0x0000_0000, 0x0, 0x0, 0x0], GroupID::G1),
            ([0x0100_0000, 0x0, 0x0, 0x0], GroupID::G2),
            ([0x0200_0000, 0x0, 0x0, 0x0], GroupID::G3),
            ([0x0300_0000, 0x0, 0x0, 0x0], GroupID::G4),
            ([0x0400_0000, 0x0, 0x0, 0x0], GroupID::G5),
            ([0x0500_0000, 0x0, 0x0, 0x0], GroupID::G6),
            ([0x0600_0000, 0x0, 0x0, 0x0], GroupID::G7),
            ([0x0700_0000, 0x0, 0x0, 0x0], GroupID::G8),
            ([0x0800_0000, 0x0, 0x0, 0x0], GroupID::G9),
            ([0x0900_0000, 0x0, 0x0, 0x0], GroupID::G10),
            ([0x0A00_0000, 0x0, 0x0, 0x0], GroupID::G11),
            ([0x0B00_0000, 0x0, 0x0, 0x0], GroupID::G12),
            ([0x0C00_0000, 0x0, 0x0, 0x0], GroupID::G13),
            ([0x0D00_0000, 0x0, 0x0, 0x0], GroupID::G14),
            ([0x0E00_0000, 0x0, 0x0, 0x0], GroupID::G15),
            ([0x0F00_0000, 0x0, 0x0, 0x0], GroupID::G16),
        ];
        for (d, g) in data_group_pairings {
            assert_eq!(Packet{data: d}.group(), g);
        }
    }
}
