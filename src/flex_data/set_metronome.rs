use crate::{
    detail::{common_properties, schema},
    flex_data::{self, UMP_MESSAGE_TYPE},
};

const STATUS: u8 = 0x2;

#[midi2_proc::generate_message(Via(crate::flex_data::FlexData), FixedSize, MinSizeUmp(3))]
struct SetMetronome {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(flex_data::FormatProperty<{flex_data::COMPLETE_FORMAT}>)]
    format: (),
    #[property(flex_data::BankProperty<{flex_data::SETUP_AND_PERFORMANCE_BANK}>)]
    bank: (),
    #[property(flex_data::StatusProperty<{STATUS}>)]
    status: (),
    #[property(flex_data::NoChannelProperty)]
    no_channel: (),
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0xFF00_0000, 0x0, 0x0>,
    >)]
    number_of_clocks_per_primary_click: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>,
    >)]
    bar_accent1: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x0000_FF00, 0x0, 0x0>,
    >)]
    bar_accent2: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x0000_00FF, 0x0, 0x0>,
    >)]
    bar_accent3: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x0, 0xFF00_0000, 0x0>,
    >)]
    number_of_subdivision_clicks1: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x0, 0x00FF_0000, 0x0>,
    >)]
    number_of_subdivision_clicks2: u8,
}

impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for SetMetronome<B> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Grouped, ux::u4};
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = SetMetronome::new_arr();
        message.set_group(u4::new(0x1));
        message.set_number_of_clocks_per_primary_click(0x9B);
        message.set_bar_accent1(0x4A);
        message.set_bar_accent2(0xFE);
        message.set_bar_accent3(0x56);
        message.set_number_of_subdivision_clicks1(0xB8);
        message.set_number_of_subdivision_clicks2(0x1B);
        assert_eq!(
            message,
            SetMetronome([0xD110_0002, 0x9B4A_FE56, 0xB81B_0000, 0x0,]),
        );
    }

    #[test]
    fn number_of_clocks_per_primary_click() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .number_of_clocks_per_primary_click(),
            0x9B,
        );
    }

    #[test]
    fn bar_accent1() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .bar_accent1(),
            0x4A,
        );
    }

    #[test]
    fn bar_accent2() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .bar_accent2(),
            0xFE,
        );
    }

    #[test]
    fn bar_accent3() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .bar_accent3(),
            0x56,
        );
    }

    #[test]
    fn number_of_subdivision_clicks1() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .number_of_subdivision_clicks1(),
            0xB8,
        );
    }

    #[test]
    fn number_of_subdivision_clicks2() {
        assert_eq!(
            SetMetronome::try_from(&[0xD110_0002, 0x9B4A_FE56, 0xB81B_0000][..])
                .unwrap()
                .number_of_subdivision_clicks2(),
            0x1B,
        );
    }
}
