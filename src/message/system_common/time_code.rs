use crate::message::system_common::TYPE_CODE as SYSTEM_COMMON_TYPE_CODE;

const OP_CODE: u32 = 0xF1;

#[midi2_attr::generate_message(Grouped)]
struct TimeCode {
    ump_type: Property<
        NumericalConstant<SYSTEM_COMMON_TYPE_CODE>,
        UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<
        NumericalConstant<OP_CODE>,
        UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
        BytesSchema<0xFF, 0x0, 0x0>,
    >,
    time_code: Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            TimeCodeMessage::builder()
                .group(u4::new(0x5))
                .time_code(u7::new(0x5F))
                .build(),
            Ok(TimeCodeMessage::Owned(TimeCodeOwned([
                0x15F1_5F00,
                0x0,
                0x0,
                0x0
            ]))),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00, 0x0, 0x0, 0x0])
                .unwrap()
                .group(),
            u4::new(0x5),
        );
    }

    #[test]
    fn time_code() {
        assert_eq!(
            TimeCodeMessage::from_data(&[0x15F1_5F00, 0x0, 0x0, 0x0])
                .unwrap()
                .time_code(),
            u7::new(0x5F),
        );
    }
}
