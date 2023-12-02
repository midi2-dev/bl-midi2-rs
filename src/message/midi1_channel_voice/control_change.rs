use crate::message::midi1_channel_voice::TYPE_CODE as MIDI1_CHANNEL_VOICE_TYPE;

const OP_CODE: u32 = 0b1011;

#[midi2_attr::generate_message(Grouped)]
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
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            ControlChangeMessage::builder()
                .group(u4::new(0xA))
                .channel(u4::new(0x7))
                .control(u7::new(0x36))
                .control_data(u7::new(0x37))
                .build(),
            Ok(ControlChangeMessage::Owned(ControlChangeOwned([
                0x2AB7_3637,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_data() {
        assert_eq!(
            ControlChangeMessage::from_data(&[0x2AB7_3637, 0x0, 0x0, 0x0])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }
}
