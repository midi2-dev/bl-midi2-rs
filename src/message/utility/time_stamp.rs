#[midi2_attr::generate_message]
struct TimeStamp {
    ump_type: Property<NumericalConstant<0x0>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<0b0010>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    time_stamp: Property<u20, UmpSchema<0x000F_FFFF, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            TimeStampMessage::builder()
                .group(u4::new(0x4))
                .time_stamp(u20::new(0xE_69AE))
                .build(),
            Ok(TimeStampMessage::Owned(TimeStampOwned([
                0x042E_69AE,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn builder_default() {
        assert_eq!(
            TimeStampMessage::builder().build(),
            Ok(TimeStampMessage::Owned(TimeStampOwned([
                0x0020_0000,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeStampMessage::from_data(&[0x0F20_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0xF),
        )
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            TimeStampMessage::from_data(&[0x0021_2345, 0x0, 0x0, 0x0])
                .unwrap()
                .time_stamp(),
            u20::new(0x12345),
        )
    }
}
