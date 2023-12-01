const OP_CODE: u32 = 0b0101;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message]
struct RelativeAssignableController {
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
            RelativeAssignableControllerMessage::builder()
                .group(u4::new(0x3))
                .channel(u4::new(0x1))
                .bank(u7::new(0x24))
                .index(u7::new(0x52))
                .controller_data(0x898874E4)
                .build(),
            Ok(RelativeAssignableControllerMessage::Owned(
                RelativeAssignableControllerOwned([0x4351_2452, 0x898874E4, 0x0, 0x0,])
            )),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            RelativeAssignableControllerMessage::from_data(&[0x4351_2452, 0x898874E4, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x3),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            RelativeAssignableControllerMessage::from_data(&[0x4351_2452, 0x898874E4, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x1),
        );
    }

    #[test]
    pub fn bank() {
        assert_eq!(
            RelativeAssignableControllerMessage::from_data(&[0x4351_2452, 0x898874E4, 0x0, 0x0])
                .unwrap()
                .bank(),
            u7::new(0x24),
        );
    }

    #[test]
    pub fn index() {
        assert_eq!(
            RelativeAssignableControllerMessage::from_data(&[0x4351_2452, 0x898874E4, 0x0, 0x0])
                .unwrap()
                .index(),
            u7::new(0x52),
        );
    }

    #[test]
    pub fn controller_data() {
        assert_eq!(
            RelativeAssignableControllerMessage::from_data(&[0x4351_2452, 0x898874E4, 0x0, 0x0])
                .unwrap()
                .controller_data(),
            0x898874E4,
        );
    }
}
