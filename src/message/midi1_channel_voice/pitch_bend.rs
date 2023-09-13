use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1110;

#[midi2_attr::generate_message]
struct PitchBend {
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
    bend: Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x7F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            PitchBendMessage::<Ump>::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0x1))
                .channel(u4::new(0xE))
                .bend(u14::new(0x147))
                .build(),
            Ok(PitchBendMessage::<Ump>(&[0x21EE_4702, 0x0, 0x0, 0x0])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            PitchBendMessage::<Ump>::from_data(&[0x21EE_4702, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PitchBendMessage::<Ump>::from_data(&[0x21EE_4702, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn bend() {
        assert_eq!(
            PitchBendMessage::<Ump>::from_data(&[0x21EE_4702, 0x0, 0x0, 0x0])
                .unwrap()
                .bend(),
            u14::new(0x147)
        );
    }

    #[test]
    fn builder_bytes() {
        assert_eq!(
            PitchBendMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .channel(u4::new(0xE))
                .bend(u14::new(0x147))
                .build(),
            Ok(PitchBendMessage::<Bytes>(&[0xEE, 0x47, 0x02])),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            PitchBendMessage::<Bytes>::from_data(&[0xEE, 0x47, 0x02])
                .unwrap()
                .channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn bend_bytes() {
        assert_eq!(
            PitchBendMessage::<Bytes>::from_data(&[0xEE, 0x47, 0x02])
                .unwrap()
                .bend(),
            u14::new(0x147)
        );
    }
}
