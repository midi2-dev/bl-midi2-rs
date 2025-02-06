use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    ux::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b1111;

/// MIDI 2.0 Channel Voice Per Note Management Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(1))]
struct PerNoteManagement {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    note_number: u7,
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

        let mut message = PerNoteManagement::<[u32; 4]>::new();
        message.set_group(u4::new(0xB));
        message.set_channel(u4::new(0x9));
        message.set_note_number(u7::new(0x1C));
        message.set_detach(true);
        message.set_reset(true);

        assert_eq!(message, PerNoteManagement([0x4BF9_1C03, 0x0, 0x0, 0x0]),);
    }

    #[test]
    fn note_number() {
        assert_eq!(
            PerNoteManagement::try_from(&[0x4BF9_1C03][..])
                .unwrap()
                .note_number(),
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
