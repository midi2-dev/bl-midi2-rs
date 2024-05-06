use crate::{
    detail::{common_properties, schema},
    flex_data::{self, UMP_MESSAGE_TYPE},
};

const STATUS: u8 = 0x1;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct SetTimeSignature {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
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
    numerator: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x00FF_0000, 0x0, 0x0>,
    >)]
    denominator: u8,
    #[property(common_properties::UmpSchemaProperty<
        u8,
        schema::Ump<0x0, 0x0000_FF00, 0x0, 0x0>,
    >)]
    number_of_32nd_notes: u8,
}

impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for SetTimeSignature<B> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{flex_data::FlexDataMessage, traits::Grouped, ux::u4};
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = SetTimeSignature::new_arr();
        message.set_group(u4::new(0xA));
        message.set_numerator(0xCD);
        message.set_denominator(0x90);
        message.set_number_of_32nd_notes(0x7E);
        assert_eq!(
            message,
            SetTimeSignature([0x0, 0xDA10_0001, 0xCD90_7E00, 0x0, 0x0,]),
        );
    }

    #[test]
    fn numerator() {
        assert_eq!(
            SetTimeSignature::try_from(&[0xDA10_0001, 0xCD90_7E00,][..])
                .unwrap()
                .numerator(),
            0xCD,
        );
    }

    #[test]
    fn denominator() {
        assert_eq!(
            SetTimeSignature::try_from(&[0xDA10_0001, 0xCD90_7E00,][..])
                .unwrap()
                .denominator(),
            0x90,
        );
    }

    #[test]
    fn number_of_32nd_notes() {
        assert_eq!(
            SetTimeSignature::try_from(&[0xDA10_0001, 0xCD90_7E00,][..])
                .unwrap()
                .number_of_32nd_notes(),
            0x7E,
        );
    }

    #[test]
    fn bank() {
        assert_eq!(
            SetTimeSignature::try_from(&[0xDA10_0001, 0xCD90_7E00,][..])
                .unwrap()
                .bank(),
            flex_data::Bank::SetupAndPerformance,
        );
    }

    #[test]
    fn status() {
        assert_eq!(
            SetTimeSignature::try_from(&[0xDA10_0001, 0xCD90_7E00,][..])
                .unwrap()
                .status(),
            STATUS,
        );
    }
}
