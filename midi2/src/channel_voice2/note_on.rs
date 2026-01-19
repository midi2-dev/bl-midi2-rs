use crate::{
    channel_voice2::{
        attribute::{Attribute, AttributeProperty},
        UMP_MESSAGE_TYPE,
    },
    detail::{common_properties, schema},
    ux::{u4, u7},
};

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
use crate::traits::{Channeled, Grouped};

/// Converts a CV2 Note On message to a CV1 Note On message.
/// Note: Due to 0 velocity Note On messages being considered
/// a Note Off in CV1 but not in CV2, a 0 velocity CV2 message
/// will be converted to a 1 velocity CV1 message.
#[cfg(feature = "channel-voice1")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32> + crate::buffer::BufferMut,
    > Into<crate::channel_voice1::NoteOn<B>> for (NoteOn<A>, crate::channel_voice1::NoteOn<B>)
{
    fn into(self) -> crate::channel_voice1::NoteOn<B> {
        let (src, mut dest) = self;
        dest.set_group(src.group());
        dest.set_channel(src.channel());
        dest.set_note_number(src.note_number());
        match src.velocity() {
            // Since 0 velocity doesn't trigger a note off in CV2 like in CV1,
            // we need to convert 0 velocity in CV2 to 1 velocity in CV1.
            // See MIDI 2.0 spec 7.4.2: MIDI 2.0 Note On Message -> Velocity
            // for details.
            0 => dest.set_velocity(u7::new(0x01)),
            _ => dest.set_velocity(u7::new((src.velocity() >> 9) as u8)),
        }
        dest
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
        use crate::channel_voice1;
        use crate::traits::{Channeled, Grouped};

        let mut message2 = NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        assert_eq!(message2, NoteOn([0x4898_5E00, 0x8000_0000, 0x0, 0x0]),);

        let mut message1 = channel_voice1::NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let mut message21 = channel_voice1::NoteOn::<[u32; 4]>::new();
        let message21: channel_voice1::NoteOn<[u32; 4]> = (message2, message21).into();

        assert_eq!(message21, message1);
    }

    #[test]
    fn into_midi_1_zero_velocity() {
        use crate::channel_voice1;
        use crate::traits::{Channeled, Grouped};

        let mut message2 = NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x0000);

        assert_eq!(message2, NoteOn([0x4898_5E00, 0x0000_0000, 0x0, 0x0]),);

        let mut message1 = channel_voice1::NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x01));

        let mut message21 = channel_voice1::NoteOn::<[u32; 4]>::new();
        let message21: channel_voice1::NoteOn<[u32; 4]> = (message2, message21).into();

        assert_eq!(message21, message1);
    }
}
