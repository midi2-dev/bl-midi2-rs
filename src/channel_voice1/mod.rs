use crate::detail::BitOps;

mod channel_pressure;
mod control_change;
mod key_pressure;
mod note_off;
mod note_on;
mod pitch_bend;
mod program_change;

pub use channel_pressure::*;
pub use control_change::*;
pub use key_pressure::*;
pub use note_off::*;
pub use note_on::*;
pub use pitch_bend::*;
pub use program_change::*;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x2;

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Channeled,
    midi2_proc::Grouped,
    midi2_proc::FromBytes,
    midi2_proc::FromUmp,
    midi2_proc::JitterReduced,
    midi2_proc::TryFromBytes,
    midi2_proc::TryFromUmp,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum ChannelVoice1<B: crate::buffer::Buffer> {
    ChannelPressure(ChannelPressure<B>),
    ControlChange(ControlChange<B>),
    KeyPressure(KeyPressure<B>),
    NoteOff(NoteOff<B>),
    NoteOn(NoteOn<B>),
    PitchBend(PitchBend<B>),
    ProgramChange(ProgramChange<B>),
}

impl<'a, U: crate::buffer::Unit> core::convert::TryFrom<&'a [U]> for ChannelVoice1<&'a [U]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [U]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };
        Ok(match status(buffer) {
            channel_pressure::STATUS => ChannelPressure::try_from(buffer)?.into(),
            control_change::STATUS => ControlChange::try_from(buffer)?.into(),
            key_pressure::STATUS => KeyPressure::try_from(buffer)?.into(),
            note_off::STATUS => NoteOff::try_from(buffer)?.into(),
            note_on::STATUS => NoteOn::try_from(buffer)?.into(),
            pitch_bend::STATUS => PitchBend::try_from(buffer)?.into(),
            program_change::STATUS => ProgramChange::try_from(buffer)?.into(),
            _ => Err(crate::error::Error::InvalidData(
                "Unknown midi1 channel voice status",
            ))?,
        })
    }
}

fn status<U: crate::buffer::Unit>(buffer: &[U]) -> u8 {
    match <U as crate::buffer::UnitPrivate>::UNIT_ID {
        crate::buffer::UNIT_ID_U8 => {
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u8(buffer)[0].nibble(0)
        }
        crate::buffer::UNIT_ID_U32 => {
            <U as crate::buffer::UnitPrivate>::specialise_buffer_u32(buffer)[0].nibble(2)
        }
        _ => unreachable!(),
    }
    .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        traits::{
            Channeled, Data, FromBytes, FromUmp, Grouped, RebufferInto, TryFromBytes, TryFromUmp,
            TryRebufferInto,
        },
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        assert_eq!(
            ChannelVoice1::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ChannelVoice1::try_from(&[0xD6_u8, 0x09_u8][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelVoice1::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .group(),
            u4::new(0xF),
        );
    }

    #[test]
    fn from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelVoice1::try_from(&buffer[..]).unwrap();
        let owned = ChannelVoice1::<std::vec::Vec<u32>>::from_bytes(borrowed);
        assert_eq!(owned.data(), &[0x20D6_0900]);
    }

    #[test]
    fn from_ump() {
        let buffer = [0x20D6_0900_u32];
        let borrowed = ChannelVoice1::try_from(&buffer[..]).unwrap();
        let owned = ChannelVoice1::<std::vec::Vec<u8>>::from_ump(borrowed);
        assert_eq!(owned.data(), &[0xD6, 0x09]);
    }

    #[test]
    fn try_from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelVoice1::try_from(&buffer[..]).unwrap();
        let owned = ChannelVoice1::<[u32; 5]>::try_from_bytes(borrowed).unwrap();
        assert_eq!(owned.data(), &[0x20D6_0900]);
    }

    #[test]
    fn try_from_ump() {
        let buffer = [0x20D6_0900_u32];
        let borrowed = ChannelVoice1::try_from(&buffer[..]).unwrap();
        let owned = ChannelVoice1::<[u8; 3]>::try_from_ump(borrowed).unwrap();
        assert_eq!(owned.data(), &[0xD6, 0x09]);
    }

    #[test]
    fn clone() {
        let buffer = [0x20D6_0900_u32];
        let borrowed = ChannelVoice1::try_from(&buffer[..]).unwrap();
        let cloned = borrowed.clone();
        assert_eq!(borrowed.data(), cloned.data());
    }

    #[test]
    fn rebuffer_into() {
        let message: ChannelVoice1<std::vec::Vec<u32>> =
            ChannelVoice1::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .rebuffer_into();
        assert_eq!(message.data(), &[0x2FD6_0900]);
    }

    #[test]
    fn try_rebuffer_into() {
        let message: ChannelVoice1<[u32; 4]> = ChannelVoice1::try_from(&[0x2FD6_0900_u32][..])
            .unwrap()
            .try_rebuffer_into()
            .unwrap();
        assert_eq!(message.data(), &[0x2FD6_0900]);
    }
}
