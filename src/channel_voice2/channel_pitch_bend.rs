use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    ux::u4,
};

pub(crate) const STATUS: u8 = 0b1110;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct ChannelPitchBend {
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
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    pitch_bend_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::traits::{Channeled, Grouped};

        let mut message = ChannelPitchBend::new_arr();
        message.set_group(u4::new(0xB));
        message.set_channel(u4::new(0x9));
        message.set_pitch_bend_data(0x08306AF8);

        assert_eq!(
            message,
            ChannelPitchBend([0x0, 0x4BE9_0000, 0x0830_6AF8, 0x0, 0x0]),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            ChannelPitchBend::try_from(&[0x4BE9_0000, 0x0830_6AF8][..])
                .unwrap()
                .pitch_bend_data(),
            0x0830_6AF8,
        );
    }
}
