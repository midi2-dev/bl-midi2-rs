const OP_CODE: u32 = 0b0001;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message(Grouped, Channeled)]
struct AssignablePerNoteController {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    index: Property<u8, UmpSchema<0x0000_00FF, 0x0, 0x0, 0x0>, ()>,
    controller_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            AssignablePerNoteControllerMessage::builder()
                .group(u4::new(0x2))
                .channel(u4::new(0x4))
                .note(u7::new(0x6F))
                .index(0xB1)
                .controller_data(0x46105EE5)
                .build(),
            Ok(AssignablePerNoteControllerMessage::Owned(
                AssignablePerNoteControllerOwned([0x4214_6FB1, 0x46105EE5, 0x0, 0x0,])
            )),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x2),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x4),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x6F),
        );
    }

    #[test]
    fn index() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5, 0x0, 0x0])
                .unwrap()
                .index(),
            0xB1,
        );
    }

    #[test]
    fn controller_data() {
        assert_eq!(
            AssignablePerNoteControllerMessage::from_data(&[0x4214_6FB1, 0x46105EE5, 0x0, 0x0])
                .unwrap()
                .controller_data(),
            0x46105EE5,
        );
    }
}
