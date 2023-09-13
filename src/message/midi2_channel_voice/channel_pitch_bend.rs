const OP_CODE: u32 = 0b1110;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct ChannelPitchBend {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    pitch_bend_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            ChannelPitchBendMessage::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0xB))
                .channel(u4::new(0x9))
                .pitch_bend_data(0x08306AF8)
                .build(),
            Ok(ChannelPitchBendMessage(&[
                0x4BE9_0000,
                0x0830_6AF8,
                0x0,
                0x0
            ])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xB),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x9),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            ChannelPitchBendMessage::from_data(&[0x4BE9_0000, 0x0830_6AF8, 0x0, 0x0])
                .unwrap()
                .pitch_bend_data(),
            0x0830_6AF8,
        );
    }
}
