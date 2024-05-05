use crate::{
    message::{common_properties, midi2_channel_voice::UMP_MESSAGE_TYPE},
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b0110;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct PerNotePitchBend {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    note: u7,
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

        let mut message = PerNotePitchBend::new_arr();
        message.set_group(u4::new(0x9));
        message.set_channel(u4::new(0x2));
        message.set_note(u7::new(0x76));
        message.set_pitch_bend_data(0x2AD74672);

        assert_eq!(
            message,
            PerNotePitchBend([0x0, 0x4962_7600, 0x2AD74672, 0x0, 0x0]),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNotePitchBend::try_from(&[0x4962_7600, 0x2AD74672][..])
                .unwrap()
                .note(),
            u7::new(0x76),
        );
    }

    #[test]
    fn pitch_bend_data() {
        assert_eq!(
            PerNotePitchBend::try_from(&[0x4962_7600, 0x2AD74672][..])
                .unwrap()
                .pitch_bend_data(),
            0x2AD74672,
        );
    }
}
