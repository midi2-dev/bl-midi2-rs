use crate::util::BitOps;

pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod pitch_bend;
pub mod program_change;

use channel_pressure::ChannelPressure;
use control_change::ControlChange;
use key_pressure::KeyPressure;
use note_off::NoteOff;
use note_on::NoteOn;
use pitch_bend::PitchBend;
use program_change::ProgramChange;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x2;

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Channeled,
    midi2_proc::Grouped,
    midi2_proc::FromBytes,
    midi2_proc::FromUmp,
    midi2_proc::TryFromBytes,
    midi2_proc::TryFromUmp,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi1ChannelVoice<B: crate::buffer::Buffer> {
    ChannelPressure(ChannelPressure<B>),
    ControlChange(ControlChange<B>),
    KeyPressure(KeyPressure<B>),
    NoteOff(NoteOff<B>),
    NoteOn(NoteOn<B>),
    PitchBend(PitchBend<B>),
    ProgramChange(ProgramChange<B>),
}

impl<'a, U: crate::buffer::Unit> core::convert::TryFrom<&'a [U]> for Midi1ChannelVoice<&'a [U]> {
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
                "Unknown channel voice status",
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
        numeric_types::*,
        traits::{Channeled, Data, FromBytes, FromUmp, Grouped, TryFromBytes, TryFromUmp},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        assert_eq!(
            Midi1ChannelVoice::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            Midi1ChannelVoice::try_from(&[0xD6_u8, 0x09_u8][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Midi1ChannelVoice::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .group(),
            u4::new(0xF),
        );
    }

    #[test]
    fn from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = Midi1ChannelVoice::try_from(&buffer[..]).unwrap();
        let owned = Midi1ChannelVoice::<std::vec::Vec<u32>>::from_bytes(borrowed);
        assert_eq!(owned.data(), &[0x20D6_0900]);
    }

    #[test]
    fn from_ump() {
        let buffer = [0x20D6_0900_u32];
        let borrowed = Midi1ChannelVoice::try_from(&buffer[..]).unwrap();
        let owned = Midi1ChannelVoice::<std::vec::Vec<u8>>::from_ump(borrowed);
        assert_eq!(owned.data(), &[0xD6, 0x09]);
    }

    #[test]
    fn try_from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = Midi1ChannelVoice::try_from(&buffer[..]).unwrap();
        let owned = Midi1ChannelVoice::<[u32; 4]>::try_from_bytes(borrowed).unwrap();
        assert_eq!(owned.data(), &[0x20D6_0900]);
    }

    #[test]
    fn try_from_ump() {
        let buffer = [0x20D6_0900_u32];
        let borrowed = Midi1ChannelVoice::try_from(&buffer[..]).unwrap();
        let owned = Midi1ChannelVoice::<[u8; 3]>::try_from_ump(borrowed).unwrap();
        assert_eq!(owned.data(), &[0xD6, 0x09]);
    }
}
