use crate::message::flex_data::{
    FlexData, SETUP_AND_PERFORMANCE_BANK, TYPE_CODE as FLEX_DATA_TYPE,
};

const STATUS: u32 = 0x1;

#[midi2_proc::generate_message(Grouped)]
struct SetTimeSignature {
    ump_type:
        Property<NumericalConstant<FLEX_DATA_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x00C0_0000, 0x0, 0x0, 0x0>, ()>,
    address: Property<NumericalConstant<0x1>, UmpSchema<0x0030_0000, 0x0, 0x0, 0x0>, ()>,
    bank: Property<
        NumericalConstant<SETUP_AND_PERFORMANCE_BANK>,
        UmpSchema<0x0000_FF00, 0x0, 0x0, 0x0>,
        (),
    >,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x0000_00FF, 0x0, 0x0, 0x0>, ()>,
    numerator: Property<u8, UmpSchema<0x0, 0xFF00_0000, 0x0, 0x0>, ()>,
    denominator: Property<u8, UmpSchema<0x0, 0x00FF_0000, 0x0, 0x0>, ()>,
    number_of_32nd_notes: Property<u8, UmpSchema<0x0, 0x0000_FF00, 0x0, 0x0>, ()>,
}

impl<'a> FlexData for SetTimeSignatureMessage<'a> {}
impl<'a> FlexData for SetTimeSignatureBorrowed<'a> {}
impl FlexData for SetTimeSignatureOwned {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SetTimeSignatureMessage::builder()
                .group(u4::new(0xA))
                .numerator(0xCD)
                .denominator(0x90)
                .number_of_32nd_notes(0x7E)
                .build(),
            Ok(SetTimeSignatureMessage::Owned(SetTimeSignatureOwned([
                0xDA10_0001,
                0xCD90_7E00,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn numerator() {
        assert_eq!(
            SetTimeSignatureMessage::from_data(&[0xDA10_0001, 0xCD90_7E00,])
                .unwrap()
                .numerator(),
            0xCD,
        );
    }

    #[test]
    fn denominator() {
        assert_eq!(
            SetTimeSignatureMessage::from_data(&[0xDA10_0001, 0xCD90_7E00,])
                .unwrap()
                .denominator(),
            0x90,
        );
    }

    #[test]
    fn number_of_32nd_notes() {
        assert_eq!(
            SetTimeSignatureMessage::from_data(&[0xDA10_0001, 0xCD90_7E00,])
                .unwrap()
                .number_of_32nd_notes(),
            0x7E,
        );
    }
}
