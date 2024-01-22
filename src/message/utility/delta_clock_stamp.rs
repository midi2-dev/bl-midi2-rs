#[midi2_proc::generate_message(Grouped)]
struct DeltaClockStamp {
    ump_type: Property<NumericalConstant<0x0>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<0b0011>, UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>, ()>,
    ticks_per_quarter_note: Property<u16, UmpSchema<0x0000_FFFF, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            DeltaClockStampMessage::builder()
                .ticks_per_quarter_note(0x704D)
                .build(),
            Ok(DeltaClockStampMessage::Owned(DeltaClockStampOwned([
                0x0030_704D,
                0x0,
                0x0,
                0x0
            ])))
        );
    }

    #[test]
    fn ticks_per_quarter_note() {
        assert_eq!(
            DeltaClockStampMessage::from_data(&[0x0030_704D])
                .unwrap()
                .ticks_per_quarter_note(),
            0x704D
        )
    }
}
