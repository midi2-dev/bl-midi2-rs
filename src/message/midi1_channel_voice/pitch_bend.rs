use crate::{
    message::{common_properties, midi1_channel_voice::UMP_MESSAGE_TYPE},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1110;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(3))]
struct PitchBend {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::numeric_types::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u14,
        schema::Bytes<0x00, 0x7F, 0x7F>,
        schema::Ump<0x0000_7F7F, 0x0, 0x0, 0x0>,
    >)]
    bend: crate::numeric_types::u14,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        numeric_types::*,
        traits::{Channeled, Grouped},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = PitchBend::new_arr();
        message.set_group(u4::new(0x1));
        message.set_channel(u4::new(0xE));
        message.set_bend(u14::new(0x147));
        assert_eq!(message, PitchBend([0x21EE_4702]));
    }

    #[test]
    fn group() {
        assert_eq!(
            PitchBend::try_from(&[0x21EE_4702_u32][..]).unwrap().group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            PitchBend::try_from(&[0x21EE_4702_u32][..])
                .unwrap()
                .channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn bend() {
        assert_eq!(
            PitchBend::try_from(&[0x21EE_4702_u32][..]).unwrap().bend(),
            u14::new(0x147)
        );
    }
}
