use crate::message::flex_data::{Banked, SETUP_AND_PERFORMANCE_BANK, TYPE_CODE as FLEX_DATA_TYPE};

const STATUS: u32 = 0x0;

#[midi2_attr::generate_message(Grouped)]
struct SetTempo {
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
    number_of_10_nanosecond_units_per_quarter_note:
        Property<u32, UmpSchema<0x0, 0xFFFF_FFFF, 0x0, 0x0>, ()>,
}

impl<'a> Banked for SetTempoMessage<'a> {}
impl<'a> Banked for SetTempoBorrowed<'a> {}
impl Banked for SetTempoOwned {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::flex_data::Bank;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SetTempoMessage::builder()
                .group(u4::new(0x7))
                .number_of_10_nanosecond_units_per_quarter_note(0xF751FE05)
                .build(),
            Ok(SetTempoMessage::Owned(SetTempoOwned([
                0xD710_0000,
                0xF751_FE05,
                0x0,
                0x0,
            ]))),
        );
    }

    #[test]
    fn number_of_10_nanosecond_units_per_quarter_note() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD710_0000, 0xF751_FE05,])
                .unwrap()
                .number_of_10_nanosecond_units_per_quarter_note(),
            0xF751FE05,
        );
    }

    #[test]
    fn bank() {
        assert_eq!(
            SetTempoMessage::from_data(&[0xD710_0000, 0xF751_FE05,])
                .unwrap()
                .bank(),
            Bank::SetupAndPerformance,
        );
    }
}
