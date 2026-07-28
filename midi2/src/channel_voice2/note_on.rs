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

/// Tries to convert a CV1 Note On message to CV2 Note On message,
/// storing the result in a pre-allocated CV2 Note On.
///
/// Will fail if the CV1 Note On has 0 Velocity, as it must be converted
/// to a CV2 Note Off message instead.
#[cfg(feature = "channel-voice1")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32> + crate::buffer::BufferMut,
    > TryFrom<(crate::channel_voice1::NoteOn<A>, NoteOn<B>)> for NoteOn<B>
{
    type Error = crate::error::InvalidData;

    fn try_from(val: (crate::channel_voice1::NoteOn<A>, NoteOn<B>)) -> Result<Self, Self::Error> {
        use crate::conversion::MinCenterMax;
        use crate::error::InvalidData;
        use crate::traits::{Channeled, Grouped};

        let (src, mut dest) = val;
        if src.velocity() == ux::u7::new(0) {
            Err(InvalidData("CV1 Note On messages with 0 veolicty should be converted to CV2 Note Off messages."))
        } else {
            dest.set_group(src.group());
            dest.set_channel(src.channel());
            dest.set_note_number(src.note_number());
            dest.set_velocity(src.velocity().mcm_upscale::<u16>());
            Ok(dest)
        }
    }
}

/// Tries to convert a CV1 Note On message to CV2 Note On message.
///
/// Will fail if the CV1 Note On has 0 Velocity, as it must be converted
/// to a CV2 Note Off message instead.
///
/// Will also fail if there is not enough room in the destination buffer to
/// allocate a new CV2 NoteOn.
///
/// Will only attempt to allocate a new CV2 Note On if the given
/// CV1 NoteOn has a non-zero velocity.
#[cfg(feature = "channel-voice1")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32>
            + crate::buffer::BufferMut
            + crate::buffer::BufferDefault
            + crate::buffer::BufferTryResize,
    > crate::TryFromCv1<crate::channel_voice1::NoteOn<A>> for NoteOn<B>
{
    type Error = crate::error::Error;

    fn try_from_cv1(val: crate::channel_voice1::NoteOn<A>) -> Result<Self, Self::Error> {
        use crate::error::InvalidData;

        if val.velocity() == ux::u7::new(0) {
            Err(Self::Error::InvalidData(InvalidData("CV1 Note On messages with 0 veolicty should be converted to CV2 Note Off messages.")))
        } else {
            let dest = NoteOn::<B>::try_new()?;
            Ok((val, dest)
                .try_into()
                .expect("Conversion should not fail. We already checked for 0 velocity."))
        }
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
    fn try_from_midi_2() {
        use crate::traits::{Channeled, Grouped, TryIntoCv2};

        let mut message2 = NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = crate::channel_voice1::NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message12: NoteOn<[u32; 4]> = message1
            .try_into_cv2()
            .expect("Conversion should not fail.");

        assert_eq!(message12, message2);
    }

    #[test]
    fn try_from_midi_2_with_dest() {
        use crate::traits::{Channeled, Grouped};

        let mut message2 = NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = crate::channel_voice1::NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message12: NoteOn<[u32; 4]> = (message1, NoteOn::<[u32; 4]>::new())
            .try_into()
            .expect("Message should convert.");

        assert_eq!(message12, message2);
    }

    #[test]
    fn try_from_midi_2_with_dest_zero_velocity() {
        use crate::error::InvalidData;
        use crate::traits::{Channeled, Grouped};

        let mut message1 = crate::channel_voice1::NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x00));

        let message12: Result<NoteOn<[u32; 4]>, InvalidData> =
            (message1, NoteOn::<[u32; 4]>::new()).try_into();

        assert_eq!(message12, Err(InvalidData("CV1 Note On messages with 0 veolicty should be converted to CV2 Note Off messages.")));
    }
}
