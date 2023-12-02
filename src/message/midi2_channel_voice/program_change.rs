const OP_CODE: u32 = 0b1100;
const MIDI2_CHANNEL_VOICE_TYPE: u32 = 0x4;

#[midi2_attr::generate_message(Grouped)]
struct ProgramChange {
    ump_type: Property<
        NumericalConstant<MIDI2_CHANNEL_VOICE_TYPE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<OP_CODE>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    channel: Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, ()>,
    program: Property<u7, UmpSchema<0x0, 0x7F00_0000, 0x0, 0x0>, ()>,
    bank: Property<Option<u14>, UmpSchema<0x0000_0001, 0x0000_7F7F, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            ProgramChangeMessage::builder()
                .group(u4::new(0xF))
                .channel(u4::new(0xE))
                .program(u7::new(0x75))
                .bank(Some(u14::new(0x1F5E)))
                .build(),
            Ok(ProgramChangeMessage::Owned(ProgramChangeOwned([
                0x4FCE_0001,
                0x7500_5E3E,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn builder_no_bank() {
        assert_eq!(
            ProgramChangeMessage::builder()
                .group(u4::new(0xF))
                .channel(u4::new(0xE))
                .program(u7::new(0x75))
                .build(),
            Ok(ProgramChangeMessage::Owned(ProgramChangeOwned([
                0x4FCE_0000,
                0x7500_0000,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xF),
        )
    }

    #[test]
    fn channel() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xE),
        )
    }

    #[test]
    fn program() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E, 0x0, 0x0])
                .unwrap()
                .program(),
            u7::new(0x75),
        )
    }

    #[test]
    fn bank() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0001, 0x7500_5E3E, 0x0, 0x0])
                .unwrap()
                .bank(),
            Some(u14::new(0x1F5E)),
        )
    }

    #[test]
    fn no_bank() {
        assert_eq!(
            ProgramChangeMessage::from_data(&[0x4FCE_0000, 0x7500_0000, 0x0, 0x0])
                .unwrap()
                .bank(),
            None,
        )
    }
}
