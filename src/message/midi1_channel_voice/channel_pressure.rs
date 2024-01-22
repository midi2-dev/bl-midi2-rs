use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1101;

#[midi2_proc::generate_message(Grouped, Channeled)]
struct ChannelPressure {
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
    pressure: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            ChannelPressureMessage::builder()
                .group(u4::new(0xF))
                .channel(u4::new(0x6))
                .pressure(u7::new(0x09))
                .build(),
            Ok(ChannelPressureMessage::Owned(ChannelPressureOwned([
                0x2FD6_0900,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xF),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
                .unwrap()
                .pressure(),
            u7::new(0x09),
        );
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
                .unwrap()
                .into_owned(),
            ChannelPressureOwned::builder()
                .group(u4::new(0xF))
                .channel(u4::new(0x6))
                .pressure(u7::new(0x09))
                .build()
                .unwrap(),
        );
    }
}
