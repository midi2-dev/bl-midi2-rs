const OP_CODE: u32 = 0b0011;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_proc::generate_message(Grouped, Channeled)]
struct AssignableController {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    bank: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, ()>,
    index: Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, ()>,
    controller_data: Property<u32, UmpSchema<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            AssignableControllerMessage::builder()
                .group(u4::new(0xC))
                .channel(u4::new(0x8))
                .bank(u7::new(0x51))
                .index(u7::new(0x38))
                .controller_data(0x3F3ADD42)
                .build(),
            Ok(AssignableControllerMessage::Owned(
                AssignableControllerOwned([0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
            )),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            AssignableControllerMessage::from_data(&[0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xC),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            AssignableControllerMessage::from_data(&[0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x8),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            AssignableControllerMessage::from_data(&[0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
                .unwrap()
                .bank(),
            u7::new(0x51),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            AssignableControllerMessage::from_data(&[0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
                .unwrap()
                .index(),
            u7::new(0x38),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            AssignableControllerMessage::from_data(&[0x4C38_5138, 0x3F3ADD42, 0x0, 0x0])
                .unwrap()
                .controller_data(),
            0x3F3ADD42,
        );
    }
}
