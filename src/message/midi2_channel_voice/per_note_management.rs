const OP_CODE: u32 = 0b1111;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_proc::generate_message(Grouped, Channeled)]
struct PerNoteManagement {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    detach: Property<bool, UmpSchema<0x0000_0001, 0x0, 0x0, 0x0>, ()>,
    reset: Property<bool, UmpSchema<0x0000_0002, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            PerNoteManagementMessage::builder()
                .group(u4::new(0xB))
                .channel(u4::new(0x9))
                .note(u7::new(0x1C))
                .detach(true)
                .reset(true)
                .build(),
            Ok(PerNoteManagementMessage::Owned(PerNoteManagementOwned([
                0x4BF9_1C03,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03, 0x0, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x1C),
        );
    }

    #[test]
    fn detach() {
        assert!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03, 0x0, 0x0, 0x0])
                .unwrap()
                .detach(),
        );
    }

    #[test]
    fn reset() {
        assert!(
            PerNoteManagementMessage::from_data(&[0x4BF9_1C03, 0x0, 0x0, 0x0])
                .unwrap()
                .reset(),
        );
    }
}
