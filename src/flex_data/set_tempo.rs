use crate::{
    detail::{common_properties, schema},
    flex_data::{self, UMP_MESSAGE_TYPE},
};

const STATUS: u8 = 0x0;

/// MIDI 2.0 Flex Data Set Tempo Message
///
/// See the [module docs](crate::flex_data) for more info.
#[midi2_proc::generate_message(Via(crate::flex_data::FlexData), FixedSize, MinSizeUmp(2))]
struct SetTempo {
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
        u32,
        schema::Ump<0x0, 0xFFFF_FFFF, 0x0, 0x0>,
    >)]
    number_of_10_nanosecond_units_per_quarter_note: u32,
}

impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for SetTempo<B> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Grouped, ux::u4};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = SetTempo::<[u32; 4]>::new();
        message.set_group(u4::new(0x7));
        message.set_number_of_10_nanosecond_units_per_quarter_note(0xF751FE05);
        assert_eq!(message, SetTempo([0xD710_0000, 0xF751_FE05, 0x0, 0x0,]),);
    }

    #[test]
    fn number_of_10_nanosecond_units_per_quarter_note() {
        assert_eq!(
            SetTempo::try_from(&[0xD710_0000, 0xF751_FE05,][..])
                .unwrap()
                .number_of_10_nanosecond_units_per_quarter_note(),
            0xF751FE05,
        );
    }
}
