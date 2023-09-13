use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1000;

#[midi2_attr::generate_message]
struct NoteOff {
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
    velocity: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::RandomBuffer;

    #[test]
    fn builder() {
        assert_eq!(
            NoteOffMessage::<Ump>::builder(&mut Ump::random_buffer::<4>())
                .group(u4::new(0x1))
                .channel(u4::new(0xA))
                .note(u7::new(0x68))
                .velocity(u7::new(0x1B))
                .build(),
            Ok(NoteOffMessage::<Ump>(&[0x218A_681B, 0x0, 0x0, 0x0])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOffMessage::<Ump>::from_data(&[0x218A_681B, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOffMessage::<Ump>::from_data(&[0x218A_681B, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xA),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOffMessage::<Ump>::from_data(&[0x218A_681B, 0x0, 0x0, 0x0])
                .unwrap()
                .note(),
            u7::new(0x68),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOffMessage::<Ump>::from_data(&[0x218A_681B, 0x0, 0x0, 0x0])
                .unwrap()
                .velocity(),
            u7::new(0x1B),
        );
    }

    #[test]
    fn builder_bytes() {
        assert_eq!(
            NoteOffMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>())
                .channel(u4::new(0xA))
                .note(u7::new(0x68))
                .velocity(u7::new(0x1B))
                .build(),
            Ok(NoteOffMessage::<Bytes>(&[0x8A, 0x68, 0x1B])),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            NoteOffMessage::<Bytes>::from_data(&[0x8A, 0x68, 0x1B])
                .unwrap()
                .channel(),
            u4::new(0xA),
        );
    }

    #[test]
    fn note_bytes() {
        assert_eq!(
            NoteOffMessage::<Bytes>::from_data(&[0x8A, 0x68, 0x1B])
                .unwrap()
                .note(),
            u7::new(0x68),
        );
    }

    #[test]
    fn velocity_bytes() {
        assert_eq!(
            NoteOffMessage::<Bytes>::from_data(&[0x8A, 0x68, 0x1B])
                .unwrap()
                .velocity(),
            u7::new(0x1B),
        );
    }
}
