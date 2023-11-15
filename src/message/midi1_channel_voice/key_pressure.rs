use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1010;

#[midi2_attr::generate_message]
struct KeyPressure {
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
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
    pressure: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            KeyPressureOwnedPrivate::<Ump>::builder()
                .group(u4::new(0xA))
                .channel(u4::new(0x3))
                .note(u7::new(0x7F))
                .pressure(u7::new(0x5C))
                .build(),
            Ok(KeyPressureOwnedPrivate::<Ump>(arr![
                0x2AA3_7F5C,
                0x0,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Ump>::from_data(&[0x2AA3_7F5C, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Ump>::from_data(&[0x2AA3_7F5C, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x3),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Ump>::from_data(&[0x2AA3_7F5C, 0x0, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x7F),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Ump>::from_data(&[0x2AA3_7F5C, 0x0, 0x0, 0x0])
                .unwrap()
                .pressure(),
            u7::new(0x5C),
        );
    }

    #[test]
    fn builder_bytes() {
        assert_eq!(
            KeyPressureOwnedPrivate::<Bytes>::builder()
                .channel(u4::new(0x3))
                .note(u7::new(0x7F))
                .pressure(u7::new(0x5C))
                .build(),
            Ok(KeyPressureOwnedPrivate::<Bytes>(arr![0xA3, 0x7F, 0x5C])),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Bytes>::from_data(&[0xA3, 0x7F, 0x5C])
                .unwrap()
                .channel(),
            u4::new(0x3),
        );
    }

    #[test]
    fn note_bytes() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Bytes>::from_data(&[0xA3, 0x7F, 0x5C])
                .unwrap()
                .note(),
            u7::new(0x7F),
        );
    }

    #[test]
    fn pressure_bytes() {
        assert_eq!(
            KeyPressureBorrowedPrivate::<Bytes>::from_data(&[0xA3, 0x7F, 0x5C])
                .unwrap()
                .pressure(),
            u7::new(0x5C),
        );
    }
}
