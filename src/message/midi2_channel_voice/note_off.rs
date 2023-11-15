use crate::message::midi2_channel_voice::attribute::Attribute;

const OP_CODE: u32 = 0b1000;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct NoteOff {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    velocity: Property<u16, UmpSchema<0x0, 0xFFFF_0000, 0x0, 0x0>, ()>,
    attribute: Property<Option<Attribute>, UmpSchema<0x0000_00FF, 0x0000_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOffOwned::builder()
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x4E))
                .velocity(0x9DE6)
                .attribute(Some(Attribute::ManufacturerSpecific(0xCC6E)))
                .build(),
            Ok(NoteOffOwned(arr![0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0]))
        );
    }

    #[test]
    fn builder_no_attribute() {
        assert_eq!(
            NoteOffOwned::builder()
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x4E))
                .velocity(0x9DE6)
                .build(),
            Ok(NoteOffOwned(arr![0x4284_4E00, 0x9DE6_0000, 0x0, 0x0]))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOffBorrowed::<Ump>::from_data(&[0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOffBorrowed::<Ump>::from_data(&[0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOffBorrowed::<Ump>::from_data(&[0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x4E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOffBorrowed::<Ump>::from_data(&[0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0])
                .unwrap()
                .velocity(),
            0x9DE6,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOffBorrowed::<Ump>::from_data(&[0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0])
                .unwrap()
                .attribute(),
            Some(Attribute::ManufacturerSpecific(0xCC6E)),
        );
    }
}
