use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1011;

#[midi2_attr::generate_message]
struct ControlChange {
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
    control: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
    control_data: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            ControlChangeOwnedPrivate::<Ump>::builder()
                .group(u4::new(0xA))
                .channel(u4::new(0x7))
                .control(u7::new(0x36))
                .control_data(u7::new(0x37))
                .build(),
            Ok(ControlChangeOwnedPrivate::<Ump>(arr![
                0x2AB7_3637,
                0x0,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Ump>::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Ump>::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Ump>::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_data() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Ump>::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }

    #[test]
    fn builder_bytes() {
        assert_eq!(
            ControlChangeOwnedPrivate::<Bytes>::builder()
                .channel(u4::new(0x7))
                .control(u7::new(0x36))
                .control_data(u7::new(0x37))
                .build(),
            Ok(ControlChangeOwnedPrivate::<Bytes>(arr![0xB7, 0x36, 0x37])),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Bytes>::from_data(&[0xB7, 0x36, 0x37])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn control_bytes() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Bytes>::from_data(&[0xB7, 0x36, 0x37])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_data_bytes() {
        assert_eq!(
            ControlChangeBorrowedPrivate::<Bytes>::from_data(&[0xB7, 0x36, 0x37])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }
}
