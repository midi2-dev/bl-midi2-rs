use crate::{
    message::{
        common_properties,
        midi2_channel_voice::{
            attribute::{Attribute, AttributeProperty},
            UMP_MESSAGE_TYPE,
        },
    },
    numeric_types::{u4, u7},
    util::schema,
};

pub(crate) const STATUS: u8 = 0b1001;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(2))]
struct NoteOn {
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
    #[property(common_properties::UmpSchemaProperty<u16, schema::Ump<0x0, 0xFFFF_0000, 0x0, 0x0>>)]
    velocity: u16,
    #[property(AttributeProperty)]
    attribute: Option<Attribute>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::numeric_types::u9;
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOn::new_arr();
        message.set_group(u4::new(0x8));
        message.set_channel(u4::new(0x8));
        message.set_note(u7::new(0x5E));
        message.set_velocity(0x6A14);
        message.set_attribute(Some(Attribute::Pitch7_9 {
            note: u7::new(0x74),
            pitch_up: u9::new(0x18A),
        }));

        assert_eq!(message, NoteOn([0x0, 0x4898_5E03, 0x6A14_E98A, 0x0, 0x0]),);
    }

    #[test]
    fn builder_no_attribute() {
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOn::new_arr();
        message.set_group(u4::new(0x8));
        message.set_channel(u4::new(0x8));
        message.set_note(u7::new(0x5E));
        message.set_velocity(0x6A14);

        assert_eq!(message, NoteOn([0x0, 0x4898_5E00, 0x6A14_0000, 0x0, 0x0]),);
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOn::try_from(&[0x4898_5E03, 0x6A14_E98A][..])
                .unwrap()
                .note(),
            u7::new(0x5E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOn::try_from(&[0x4898_5E03, 0x6A14_E98A][..])
                .unwrap()
                .velocity(),
            0x6A14,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOn::try_from(&[0x4898_5E03, 0x6A14_E98A][..])
                .unwrap()
                .attribute(),
            Some(Attribute::Pitch7_9 {
                note: u7::new(0x74),
                pitch_up: crate::numeric_types::u9::new(0x18A),
            }),
        );
    }
}
