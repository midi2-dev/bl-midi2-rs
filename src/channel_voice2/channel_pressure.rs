use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    numeric_types::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b1101;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct ChannelPressure {
    #[property(crate::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    bank: u7,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>>)]
    index: u7,
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    channel_pressure_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setter() {
        use crate::traits::{Channeled, Grouped};

        let mut message = ChannelPressure::new_arr();
        message.set_group(u4::new(0xE));
        message.set_channel(u4::new(0xD));
        message.set_channel_pressure_data(0xDE0DE0F2);

        assert_eq!(
            message,
            ChannelPressure([0x0, 0x4EDD_0000, 0xDE0D_E0F2, 0x0, 0x0]),
        );
    }

    #[test]
    fn channel_pressure_data() {
        assert_eq!(
            ChannelPressure::try_from(&[0x4EDD_0000, 0xDE0D_E0F2][..])
                .unwrap()
                .channel_pressure_data(),
            0xDE0DE0F2,
        );
    }
}
