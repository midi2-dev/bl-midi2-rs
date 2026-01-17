use crate::{
    channel_voice2::{
        attribute::{Attribute, AttributeProperty},
        UMP_MESSAGE_TYPE,
    },
    detail::{common_properties, schema},
    ux::{u4, u7},
};

use crate::traits::{Channeled, Grouped};
use crate::{channel_voice1::NoteOn as NoteOn1, Data};
pub(crate) const STATUS: u8 = 0b1001;

/// MIDI 2.0 Channel Voice Note On Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct NoteOn {
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

#[cfg(feature = "channel-voice1")]
impl<const N: usize> Into<NoteOn1<[u32; N]>> for NoteOn<[u32; N]> {
    fn into(self) -> NoteOn1<[u32; N]> {
        let mut message = NoteOn1::<[u32; N]>::new();
        message.set_group(self.group());
        message.set_channel(self.channel());
        message.set_note_number(self.note_number());
        match self.velocity() {
            0 => message.set_velocity(u7::new(0x01)),
            _ => message.set_velocity(u7::new((self.velocity() >> 9) as u8)),
        }
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        use crate::num::Fixed7_9;
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOn::<[u32; 4]>::new();
        message.set_group(u4::new(0x8));
        message.set_channel(u4::new(0x8));
        message.set_note_number(u7::new(0x5E));
        message.set_velocity(0x6A14);
        message.set_attribute(Some(Attribute::Pitch7_9(Fixed7_9::from_bits(
            0b1110100110001010,
        ))));

        assert_eq!(message, NoteOn([0x4898_5E03, 0x6A14_E98A, 0x0, 0x0]),);
    }

    #[test]
    fn builder_no_attribute() {
        use crate::traits::{Channeled, Grouped};

        let mut message = NoteOn::<[u32; 4]>::new();
        message.set_group(u4::new(0x8));
        message.set_channel(u4::new(0x8));
        message.set_note_number(u7::new(0x5E));
        message.set_velocity(0x6A14);

        assert_eq!(message, NoteOn([0x4898_5E00, 0x6A14_0000, 0x0, 0x0]),);
    }

    #[test]
    fn note_number() {
        assert_eq!(
            NoteOn::try_from(&[0x4898_5E03, 0x6A14_E98A][..])
                .unwrap()
                .note_number(),
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
        use crate::num::Fixed7_9;

        assert_eq!(
            NoteOn::try_from(&[0x4898_5E03, 0x6A14_E98A][..])
                .unwrap()
                .attribute(),
            Some(Attribute::Pitch7_9(Fixed7_9::from_bits(0b1110100110001010))),
        );
    }

    #[test]
    fn into_midi_1() {
        use crate::traits::{Channeled, Grouped};

        let mut message2 = NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        assert_eq!(message2, NoteOn([0x4898_5E00, 0x8000_0000, 0x0, 0x0]),);

        let mut message1 = NoteOn1::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21: NoteOn1<[u32; 4]> = message2.into();

        assert_eq!(message21, message1);
    }
}
