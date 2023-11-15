use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1100;

#[midi2_attr::generate_message]
struct ProgramChange {
    ump_type: Property<
        NumericalConstant<MIDI1_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<
        NumericalConstant<OP_CODE>,
        UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
        BytesSchema<0xF0, 0x0, 0x0>,
    >,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, BytesSchema<0x0F, 0x0, 0x0>>,
    program: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            ProgramChangeOwnedPrivate::<Ump>::builder()
                .group(u4::new(0x4))
                .channel(u4::new(0x7))
                .program(u7::new(0x63))
                .build(),
            Ok(ProgramChangeOwnedPrivate::<Ump>(arr![
                0x24C7_6300,
                0x0,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ProgramChangeBorrowedPrivate::<Ump>::from_data(&[0x24C7_6300, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x4),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ProgramChangeBorrowedPrivate::<Ump>::from_data(&[0x24C7_6300, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn program() {
        assert_eq!(
            ProgramChangeBorrowedPrivate::<Ump>::from_data(&[0x24C7_6300, 0x0, 0x0, 0x0])
                .unwrap()
                .program(),
            u7::new(0x63),
        );
    }

    #[test]
    fn builder_bytes() {
        assert_eq!(
            ProgramChangeOwnedPrivate::<Bytes>::builder()
                .channel(u4::new(0x7))
                .program(u7::new(0x63))
                .build(),
            Ok(ProgramChangeOwnedPrivate::<Bytes>(arr![0xC7, 0x63, 0x00])),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ProgramChangeBorrowedPrivate::<Bytes>::from_data(&[0xC7, 0x63, 0x00])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn program_bytes() {
        assert_eq!(
            ProgramChangeBorrowedPrivate::<Bytes>::from_data(&[0xC7, 0x63, 0x00])
                .unwrap()
                .program(),
            u7::new(0x63),
        );
    }
}
