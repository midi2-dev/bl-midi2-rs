use crate::{
    channel_voice2::{
        attribute::{Attribute, AttributeProperty},
        UMP_MESSAGE_TYPE,
    },
    detail::{common_properties, schema},
    ux::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b1000;

/// MIDI 2.0 Channel Voice Note Off Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct NoteOff {
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
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOff::<[u32; 4]>::new();
        message.set_group(u4::new(0x2));
        message.set_channel(u4::new(0x4));
        message.set_note_number(u7::new(0x4E));
        message.set_velocity(0x9DE6);
        message.set_attribute(Some(Attribute::ManufacturerSpecific(0xCC6E)));

        assert_eq!(message, NoteOff([0x4284_4E01, 0x9DE6_CC6E, 0x0, 0x0]),);
    }

    #[test]
    fn builder_no_attribute() {
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOff::<[u32; 4]>::new();
        message.set_group(u4::new(0x2));
        message.set_channel(u4::new(0x4));
        message.set_note_number(u7::new(0x4E));
        message.set_velocity(0x9DE6);

        assert_eq!(message, NoteOff([0x4284_4E00, 0x9DE6_0000, 0x0, 0x0]),);
    }

    #[test]
    fn note_number() {
        assert_eq!(
            NoteOff::try_from(&[0x4284_4E01, 0x9DE6_CC6E][..])
                .unwrap()
                .note_number(),
            u7::new(0x4E),
        );
    }

    #[test]
    fn volocity() {
        assert_eq!(
            NoteOff::try_from(&[0x4284_4E01, 0x9DE6_CC6E][..])
                .unwrap()
                .velocity(),
            0x9DE6,
        );
    }

    #[test]
    fn attribute() {
        assert_eq!(
            NoteOff::try_from(&[0x4284_4E01, 0x9DE6_CC6E][..])
                .unwrap()
                .attribute(),
            Some(Attribute::ManufacturerSpecific(0xCC6E)),
        );
    }
}
