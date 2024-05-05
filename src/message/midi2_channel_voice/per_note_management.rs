use crate::{
    message::{common_properties, midi2_channel_voice::UMP_MESSAGE_TYPE},
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1111;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1))]
struct PerNoteManagement {
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
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0000_0001, 0x0, 0x0, 0x0>>)]
    detach: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0x0000_0002, 0x0, 0x0, 0x0>>)]
    reset: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        use crate::traits::{Channeled, Grouped};

        let mut message = PerNoteManagement::new_arr();
        message.set_group(u4::new(0xB));
        message.set_channel(u4::new(0x9));
        message.set_note(u7::new(0x1C));
        message.set_detach(true);
        message.set_reset(true);

        assert_eq!(
            message,
            PerNoteManagement([0x0, 0x4BF9_1C03, 0x0, 0x0, 0x0]),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            PerNoteManagement::try_from(&[0x4BF9_1C03][..])
                .unwrap()
                .note(),
            u7::new(0x1C),
        );
    }

    #[test]
    fn detach() {
        assert!(PerNoteManagement::try_from(&[0x4BF9_1C03][..])
            .unwrap()
            .detach(),);
    }

    #[test]
    fn reset() {
        assert!(PerNoteManagement::try_from(&[0x4BF9_1C03][..])
            .unwrap()
            .reset(),);
    }
}
