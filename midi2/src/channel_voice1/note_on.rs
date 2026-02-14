use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1001;

/// MIDI 1.0 Channel Voice Note On Message
///
/// See the [module docs](crate::channel_voice1) for more info.
#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct NoteOn {
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

/// Converts a CV2 Note On message to CV1 Note On message,
/// storing the result in a pre-instantiated CV1 Note On.
///
/// Note: Due to 0 velocity Note On messages being considered
/// a Note Off in CV1 but not in CV2, a 0 velocity CV2 message
/// will be converted to a 1 velocity CV1 message.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32> + crate::buffer::BufferMut,
    > From<(crate::channel_voice2::NoteOn<A>, NoteOn<B>)> for NoteOn<B>
{
    fn from(val: (crate::channel_voice2::NoteOn<A>, NoteOn<B>)) -> Self {
        use crate::conversion::MinCenterMax;
        use crate::traits::{Channeled, Grouped};

        let (src, mut dest) = val;
        dest.set_group(src.group());
        dest.set_channel(src.channel());
        dest.set_note_number(src.note_number());
        match src.velocity() {
            // Since 0 velocity doesn't trigger a note off in CV2 like in CV1,
            // we need to convert 0 velocity in CV2 to 1 velocity in CV1.
            // See MIDI 2.0 spec 7.4.2: MIDI 2.0 Note On Message -> Velocity
            // for details.
            0 => dest.set_velocity(ux::u7::new(0x01)),
            _ => dest.set_velocity(src.velocity().mcm_downscale::<ux::u7>()),
        }
        dest
    }
}

/// Tries to Convert a CV2 Note On message to a CV1 Note On message.
///
/// Note: Due to 0 velocity Note On messages being considered
/// a Note Off in CV1 but not in CV2, a 0 velocity CV2 message
/// will be converted to a 1 velocity CV1 message.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32>
            + crate::buffer::BufferMut
            + crate::buffer::BufferDefault
            + crate::buffer::BufferTryResize,
    > crate::conversion::TryFromCv2<crate::channel_voice2::NoteOn<A>> for NoteOn<B>
{
    type Error = crate::error::BufferOverflow;
    fn try_from_cv2(val: crate::channel_voice2::NoteOn<A>) -> Result<Self, Self::Error> {
        let dest = NoteOn::<B>::try_new()?;
        Ok((val, dest).into())
    }
}

/// Converts a CV2 Note On message to a CV1 Note On message.
/// This is only infallible for resizable buffers.
/// For fixed size buffers, see the Into impl for (CV2, CV1).
///
/// Note: Due to 0 velocity Note On messages being considered
/// a Note Off in CV1 but not in CV2, a 0 velocity CV2 message
/// will be converted to a 1 velocity CV1 message.
#[cfg(feature = "channel-voice2")]
impl<
        A: crate::buffer::Buffer<Unit = u32>,
        B: crate::buffer::Buffer<Unit = u32>
            + crate::buffer::BufferMut
            + crate::buffer::BufferDefault
            + crate::buffer::BufferResize,
    > crate::conversion::FromCv2<crate::channel_voice2::NoteOn<A>> for NoteOn<B>
{
    fn from_cv2(val: crate::channel_voice2::NoteOn<A>) -> Self {
        let dest = NoteOn::<B>::new();
        (val, dest).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{Channeled, Grouped},
        ux::*,
    };
    use pretty_assertions::assert_eq;

    static_assertions::assert_impl_all!(NoteOn<&[u32]>: Clone);
    static_assertions::assert_impl_all!(NoteOn<&[u32]>: Copy);

    #[test]
    fn setters() {
        let mut message = NoteOn::<[u32; 4]>::new();
        message.set_group(u4::new(0xD));
        message.set_channel(u4::new(0xE));
        message.set_note_number(u7::new(0x75));
        message.set_velocity(u7::new(0x3D));
        assert_eq!(message, NoteOn([0x2D9E_753D, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = NoteOn::<[u8; 3]>::new();

        message.set_channel(u4::new(0xE));
        message.set_note_number(u7::new(0x75));
        message.set_velocity(u7::new(0x3D));

        assert_eq!(message, NoteOn([0x9E, 0x75, 0x3D]));
    }

    #[test]
    fn group() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().group(),
            u4::new(0xD),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().channel(),
            u4::new(0xE),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..])
                .unwrap()
                .note_number(),
            u7::new(0x75),
        );
    }

    #[test]
    fn velocity() {
        assert_eq!(
            NoteOn::try_from(&[0x2D9E_753D_u32][..]).unwrap().velocity(),
            u7::new(0x3D),
        );
    }

    #[test]
    fn from_midi_2() {
        use crate::channel_voice2;
        use crate::conversion::IntoCv1;
        use crate::traits::{Channeled, Grouped};
        use std::vec::Vec;

        let mut message2 = crate::channel_voice2::NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOn::<Vec<u32>>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21: NoteOn<Vec<u32>> = message2.into_cv1();

        assert_eq!(message21, message1);
    }

    #[test]
    fn try_from_midi_2() {
        use crate::channel_voice2;
        use crate::conversion::TryIntoCv1;
        use crate::traits::{Channeled, Grouped};
        use std::vec::Vec;

        let mut message2 = crate::channel_voice2::NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21: NoteOn<[u32; 4]> = message2
            .try_into_cv1()
            .expect("Conversion should not fail.");

        assert_eq!(message21, message1);
    }

    #[test]
    fn from_midi_2_zero_velocity() {
        use crate::channel_voice2;
        use crate::conversion::IntoCv1;
        use crate::traits::{Channeled, Grouped};
        use std::vec::Vec;

        let mut message2 = crate::channel_voice2::NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x0000);

        let mut message1 = NoteOn::<Vec<u32>>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x01));

        let message21: NoteOn<Vec<u32>> = message2.into_cv1();

        assert_eq!(message21, message1);
    }

    #[test]
    fn from_midi_2_with_dest() {
        use crate::channel_voice2;
        use crate::traits::{Channeled, Grouped};

        let mut message2 = crate::channel_voice2::NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x8000);

        let mut message1 = NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x40));

        let message21 = NoteOn::<[u32; 4]>::new();
        let message21: NoteOn<[u32; 4]> = (message2, message21).into();

        assert_eq!(message21, message1);
    }

    #[test]
    fn from_midi_2_zero_velocity_with_dest() {
        use crate::channel_voice2;
        use crate::traits::{Channeled, Grouped};

        let mut message2 = crate::channel_voice2::NoteOn::<[u32; 4]>::new();
        message2.set_group(u4::new(0x8));
        message2.set_channel(u4::new(0x8));
        message2.set_note_number(u7::new(0x5E));
        message2.set_velocity(0x0000);

        let mut message1 = NoteOn::<[u32; 4]>::new();
        message1.set_group(u4::new(0x8));
        message1.set_channel(u4::new(0x8));
        message1.set_note_number(u7::new(0x5E));
        message1.set_velocity(u7::new(0x01));

        let message21 = NoteOn::<[u32; 4]>::new();
        let message21: NoteOn<[u32; 4]> = (message2, message21).into();

        assert_eq!(message21, message1);
    }
}
