const OP_CODE: u32 = 0b0010;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct RegisteredController {
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
            RegisteredControllerMessage::builder()
                .group(u4::new(0xA))
                .channel(u4::new(0xB))
                .bank(u7::new(0x7D))
                .index(u7::new(0x64))
                .controller_data(0x46845E00)
                .build(),
            Ok(RegisteredControllerMessage::Owned(
                RegisteredControllerOwned([0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
            )),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xB),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .bank(),
            u7::new(0x7D),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .index(),
            u7::new(0x64),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .controller_data(),
            0x46845E00,
        );
    }

    #[test]
    pub fn data() {
        assert_eq!(
            RegisteredControllerMessage::from_data(&[0x4A2B_7D64, 0x46845E00, 0x0, 0x0])
                .unwrap()
                .data(),
            &[0x4A2B_7D64, 0x46845E00, 0x0, 0x0],
        );
    }
}
