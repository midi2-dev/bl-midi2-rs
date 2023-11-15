const OP_CODE: u32 = 0b1101;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct ChannelPressure {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    channel_pressure_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use generic_array::arr;

    #[test]
    fn builder() {
        assert_eq!(
            ChannelPressureOwnedPrivate::builder()
                .group(u4::new(0xE))
                .channel(u4::new(0xD))
                .channel_pressure_data(0xDE0DE0F2)
                .build(),
            Ok(ChannelPressureOwnedPrivate(arr![
                0x4EDD_0000,
                0xDE0D_E0F2,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressureBorrowedPrivate::<Ump>::from_data(&[0x4EDD_0000, 0xDE0D_E0F2, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xE),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressureBorrowedPrivate::<Ump>::from_data(&[0x4EDD_0000, 0xDE0D_E0F2, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel_pressure_data() {
        assert_eq!(
            ChannelPressureBorrowedPrivate::<Ump>::from_data(&[0x4EDD_0000, 0xDE0D_E0F2, 0x0, 0x0])
                .unwrap()
                .channel_pressure_data(),
            0xDE0DE0F2,
        );
    }
}
