use crate::message::midi2_channel_voice::attribute::Attribute;

const OP_CODE: u32 = 0b1001;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct NoteOn {
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
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOnMessage::builder()
                .group(u4::new(0x8))
                .channel(u4::new(0x8))
                .note(u7::new(0x5E))
                .velocity(0x6A14)
                .attribute(Some(Attribute::Pitch7_9 {
                    note: u7::new(0x74),
                    pitch_up: u9::new(0x18A),
                }))
                .build(),
            Ok(NoteOnMessage::Owned(NoteOnOwned([
                0x4898_5E03,
                0x6A14_E98A,
                0x0,
                0x0
            ])))
        );
    }

    #[test]
    fn builder_no_attribute() {
        assert_eq!(
            NoteOnMessage::builder()
                .group(u4::new(0x8))
                .channel(u4::new(0x8))
                .note(u7::new(0x5E))
                .velocity(0x6A14)
                .build(),
            Ok(NoteOnMessage::Owned(NoteOnOwned([
                0x4898_5E00,
                0x6A14_0000,
                0x0,
                0x0
            ])))
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x8),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x8),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x5E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A, 0x0, 0x0])
                .unwrap()
                .velocity(),
            0x6A14,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOnMessage::from_data(&[0x4898_5E03, 0x6A14_E98A, 0x0, 0x0])
                .unwrap()
                .attribute(),
            Some(Attribute::Pitch7_9 {
                note: u7::new(0x74),
                pitch_up: u9::new(0x18A),
            }),
        );
    }
    #[test]
    fn to_owned() {
        let _ = {
            let buffer = [0x4898_5E03, 0x6A14_E98A, 0x0, 0x0];
            let borrowed = NoteOnMessage::from_data(&buffer).unwrap();
            borrowed.to_owned();
        };
    }
}
