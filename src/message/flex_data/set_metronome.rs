use crate::message::flex_data::{
    FlexData, SETUP_AND_PERFORMANCE_BANK, TYPE_CODE as FLEX_DATA_TYPE,
};

const STATUS: u32 = 0x2;

#[midi2_attr::generate_message(Grouped)]
struct SetMetronome {
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
    number_of_clocks_per_primary_click: Property<u8, UmpSchema<0x0, 0xFF00_0000, 0x0, 0x0>, ()>,
    bar_accent1: Property<u8, UmpSchema<0x0, 0x00FF_0000, 0x0, 0x0>, ()>,
    bar_accent2: Property<u8, UmpSchema<0x0, 0x0000_FF00, 0x0, 0x0>, ()>,
    bar_accent3: Property<u8, UmpSchema<0x0, 0x0000_00FF, 0x0, 0x0>, ()>,
    number_of_subdivision_clicks1: Property<u8, UmpSchema<0x0, 0x0, 0xFF00_0000, 0x0>, ()>,
    number_of_subdivision_clicks2: Property<u8, UmpSchema<0x0, 0x0, 0x00FF_0000, 0x0>, ()>,
}

impl<'a> FlexData for SetMetronomeMessage<'a> {}
impl<'a> FlexData for SetMetronomeBorrowed<'a> {}
impl FlexData for SetMetronomeOwned {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            SetMetronomeMessage::builder()
                .group(u4::new(0x1))
                .number_of_clocks_per_primary_click(0x9B)
                .bar_accent1(0x4A)
                .bar_accent2(0xFE)
                .bar_accent3(0x56)
                .number_of_subdivision_clicks1(0xB8)
                .number_of_subdivision_clicks2(0x1B)
                .build(),
            Ok(SetMetronomeMessage::Owned(SetMetronomeOwned([
                0xD110_0002,
                0x9B4A_FE56,
                0xB81B_0000,
                0x0,
            ]))),
        );
    }

    #[test]
    fn number_of_clocks_per_primary_click() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .number_of_clocks_per_primary_click(),
            0x9B,
        );
    }

    #[test]
    fn bar_accent1() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .bar_accent1(),
            0x4A,
        );
    }

    #[test]
    fn bar_accent2() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .bar_accent2(),
            0xFE,
        );
    }

    #[test]
    fn bar_accent3() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .bar_accent3(),
            0x56,
        );
    }

    #[test]
    fn number_of_subdivision_clicks1() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .number_of_subdivision_clicks1(),
            0xB8,
        );
    }

    #[test]
    fn number_of_subdivision_clicks2() {
        assert_eq!(
            SetMetronomeMessage::from_data(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000])
                .unwrap()
                .number_of_subdivision_clicks2(),
            0x1B,
        );
    }
}
