const OP_CODE: u32 = 0b0110;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct PerNotePitchBend {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    note: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    pitch_bend_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            PerNotePitchBendMessage::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0x9))
                .channel(u4::new(0x2))
                .note(u7::new(0x76))
                .pitch_bend_data(0x2AD74672)
                .build(),
            Ok(PerNotePitchBendMessage(&[
                0x4962_7600,
                0x2AD74672,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x9),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x2),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x76),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            PerNotePitchBendMessage::from_data(&[0x4962_7600, 0x2AD74672, 0x0, 0x0])
                .unwrap()
                .pitch_bend_data(),
            0x2AD74672,
        );
    }
}
