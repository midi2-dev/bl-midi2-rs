use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1000;

/// MIDI 1.0 Channel Voice Note Off Message
///
/// See the [module docs](crate::channel_voice1) for more info.
#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct NoteOff {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::ux::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    note_number: crate::ux::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    velocity: crate::ux::u7,
}

/// Converts a CV2 Note Off message to CV1 Note Off message,
/// storing the result in a pre-allocated CV1 Note Off.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32> + crate::buffer::BufferMut,
    > From<(crate::channel_voice2::NoteOff<A>, NoteOff<B>)> for NoteOff<B>
{
    fn from(val: (crate::channel_voice2::NoteOff<A>, NoteOff<B>)) -> Self {
        use crate::conversion::MinCenterMax;
        use crate::traits::{Channeled, Grouped};

        let (src, mut dest) = val;
        dest.set_group(src.group());
        dest.set_channel(src.channel());
        dest.set_note_number(src.note_number());
        dest.set_velocity(src.velocity().mcm_downscale::<ux::u7>());
        dest
    }
}

/// Converts a CV2 Note Off message to a CV1 Note Off message.
/// This is only infallible for resizable buffers.
/// For fixed size buffers, see TryFromCv2.
///
/// Note: Due to 0 velocity Note Off messages being considered
/// a Note Off in CV1 but not in CV2, a 0 velocity CV2 message
/// will be converted to a 1 velocity CV1 message.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32>
            + crate::buffer::BufferMut
            + crate::buffer::BufferDefault
            + crate::buffer::BufferResize,
    > crate::FromCv2<crate::channel_voice2::NoteOff<A>> for NoteOff<B>
{
    fn from_cv2(val: crate::channel_voice2::NoteOff<A>) -> Self {
        let dest = NoteOff::<B>::new();
        (val, dest).into()
    }
}

/// Tries to Convert a CV2 Note Off message to a CV1 Note Off message.
/// Fails if the underlying buffer doesn't have enough room for a new Note Off message.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32>
            + crate::buffer::BufferMut
            + crate::buffer::BufferDefault
            + crate::buffer::BufferTryResize,
    > crate::TryFromCv2<crate::channel_voice2::NoteOff<A>> for NoteOff<B>
{
    type Error = crate::error::BufferOverflow;
    fn try_from_cv2(val: crate::channel_voice2::NoteOff<A>) -> Result<Self, Self::Error> {
        let dest = NoteOff::<B>::try_new()?;
        Ok((val, dest).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{Channeled, Grouped, IntoCv1, TryIntoCv1},
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = NoteOff::<[u32; 4]>::new();
        message.set_group(u4::new(0x1));
        message.set_channel(u4::new(0xA));
        message.set_note_number(u7::new(0x68));
        message.set_velocity(u7::new(0x1B));
        assert_eq!(message, NoteOff([0x218A_681B, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..]).unwrap().group(),
            u4::new(0x1),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..]).unwrap().channel(),
            u4::new(0xA),
        );
    }

    #[test]
    fn note_number() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..])
                .unwrap()
                .note_number(),
            u7::new(0x68),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOff::try_from(&[0x218A_681B_u32][..])
                .unwrap()
                .velocity(),
            u7::new(0x1B),
        );
    }

    #[test]
    fn from_midi_2_with_dest() {
        use crate::traits::{Channeled, Grouped};

        let mut message2 = crate::channel_voice2::NoteOff::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOff::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21 = NoteOff::<[u32; 4]>::new();
        let message21: NoteOff<[u32; 4]> = (message2, message21).into();

        assert_eq!(message21, message1);
    }

    #[test]
    fn from_midi_2() {
        use crate::traits::{Channeled, Grouped};
        use std::vec::Vec;

        let mut message2 = crate::channel_voice2::NoteOff::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOff::<Vec<u32>>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21: NoteOff<Vec<u32>> = message2.into_cv1();

        assert_eq!(message21, message1);
    }

    #[test]
    fn try_from_midi_2() {
        use crate::traits::{Channeled, Grouped};

        let mut message2 = crate::channel_voice2::NoteOff::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOff::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21: NoteOff<[u32; 4]> = message2
            .try_into_cv1()
            .expect("Conversion should not fail.");

        assert_eq!(message21, message1);
    }
}
