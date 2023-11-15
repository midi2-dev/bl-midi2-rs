pub mod tune_request {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct TuneRequest {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xF6>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod timing_clock {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct TimingClock {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xF8>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod start {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct Start {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFA>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod cont {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct Continue {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFB>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod stop {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct Stop {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFC>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod active_sensing {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct ActiveSensing {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFE>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}
pub mod reset {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    #[midi2_attr::generate_message]
    struct Reset {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFF>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }
}

#[cfg(test)]
mod tests {
    use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;
    use generic_array::arr;

    #[midi2_attr::generate_message]
    struct Test {
        ump_type: Property<
            NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
            UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
            (),
        >,
        status: Property<
            NumericalConstant<0xFF>,
            UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
            BytesSchema<0xFF, 0x0, 0x0>,
        >,
    }

    #[test]
    fn builder() {
        assert_eq!(
            TestOwned::<Ump>::builder().group(u4::new(0x9)).build(),
            Ok(TestOwned::<Ump>(arr![0x19FF_0000, 0x0, 0x0, 0x0])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TestBorrowed::<Ump>::from_data(&[0x19FF_0000, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x9),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            TestOwned::<Bytes>::builder().build(),
            Ok(TestOwned::<Bytes>(arr![0xFF, 0x0, 0x0])),
        );
    }

    #[test]
    fn bytes_from_data() {
        assert_eq!(
            TestBorrowed::<Bytes>::from_data(&[0xFF, 0x0, 0x0]),
            Ok(TestBorrowed::<Bytes>(&[0xFF, 0x0, 0x0])),
        )
    }
}
