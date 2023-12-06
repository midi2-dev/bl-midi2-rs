use crate::{util::BitOps, *};

pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod pitch_bend;
pub mod program_change;

pub const TYPE_CODE: u32 = 0x2;

use channel_pressure::ChannelPressureBorrowed;
use channel_pressure::ChannelPressureBuilder;
use channel_pressure::ChannelPressureMessage;
use channel_pressure::ChannelPressureOwned;
use control_change::ControlChangeBorrowed;
use control_change::ControlChangeBuilder;
use control_change::ControlChangeMessage;
use control_change::ControlChangeOwned;
use key_pressure::KeyPressureBorrowed;
use key_pressure::KeyPressureBuilder;
use key_pressure::KeyPressureMessage;
use key_pressure::KeyPressureOwned;
use note_off::NoteOffBorrowed;
use note_off::NoteOffBuilder;
use note_off::NoteOffMessage;
use note_off::NoteOffOwned;
use note_on::NoteOnBorrowed;
use note_on::NoteOnBuilder;
use note_on::NoteOnMessage;
use note_on::NoteOnOwned;
use pitch_bend::PitchBendBorrowed;
use pitch_bend::PitchBendBuilder;
use pitch_bend::PitchBendMessage;
use pitch_bend::PitchBendOwned;
use program_change::ProgramChangeBorrowed;
use program_change::ProgramChangeBuilder;
use program_change::ProgramChangeMessage;
use program_change::ProgramChangeOwned;

#[derive(
    derive_more::From,
    midi2_attr::Data,
    midi2_attr::Channeled,
    midi2_attr::Grouped,
    midi2_attr::WriteByteData,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi1ChannelVoiceMessage<'a> {
    ChannelPressure(ChannelPressureMessage<'a>),
    ControlChange(ControlChangeMessage<'a>),
    KeyPressure(KeyPressureMessage<'a>),
    NoteOff(NoteOffMessage<'a>),
    NoteOn(NoteOnMessage<'a>),
    PitchBend(PitchBendMessage<'a>),
    ProgramChange(ProgramChangeMessage<'a>),
}

#[derive(
    derive_more::From,
    midi2_attr::Data,
    midi2_attr::Channeled,
    midi2_attr::Grouped,
    midi2_attr::WriteByteData,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi1ChannelVoiceBorrowed<'a> {
    ChannelPressure(ChannelPressureBorrowed<'a>),
    ControlChange(ControlChangeBorrowed<'a>),
    KeyPressure(KeyPressureBorrowed<'a>),
    NoteOff(NoteOffBorrowed<'a>),
    NoteOn(NoteOnBorrowed<'a>),
    PitchBend(PitchBendBorrowed<'a>),
    ProgramChange(ProgramChangeBorrowed<'a>),
}

#[derive(
    derive_more::From,
    midi2_attr::Data,
    midi2_attr::Channeled,
    midi2_attr::Grouped,
    midi2_attr::WriteByteData,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi1ChannelVoiceOwned {
    ChannelPressure(ChannelPressureOwned),
    ControlChange(ControlChangeOwned),
    KeyPressure(KeyPressureOwned),
    NoteOff(NoteOffOwned),
    NoteOn(NoteOnOwned),
    PitchBend(PitchBendOwned),
    ProgramChange(ProgramChangeOwned),
}

#[derive(Default)]
pub struct Midi1ChannelVoiceBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<ChannelPressureOwned>
        + core::convert::From<ControlChangeOwned>
        + core::convert::From<KeyPressureOwned>
        + core::convert::From<NoteOffOwned>
        + core::convert::From<NoteOnOwned>
        + core::convert::From<PitchBendOwned>
        + core::convert::From<ProgramChangeOwned>;

impl<M> Midi1ChannelVoiceBuilder<M>
where
    M: core::convert::From<ChannelPressureOwned>
        + core::convert::From<ControlChangeOwned>
        + core::convert::From<KeyPressureOwned>
        + core::convert::From<NoteOffOwned>
        + core::convert::From<NoteOnOwned>
        + core::convert::From<PitchBendOwned>
        + core::convert::From<ProgramChangeOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn channel_pressure(self) -> ChannelPressureBuilder<M> {
        ChannelPressureBuilder::new()
    }
    pub fn control_change(self) -> ControlChangeBuilder<M> {
        ControlChangeBuilder::new()
    }
    pub fn key_pressure(self) -> KeyPressureBuilder<M> {
        KeyPressureBuilder::new()
    }
    pub fn note_off(self) -> NoteOffBuilder<M> {
        NoteOffBuilder::new()
    }
    pub fn note_on(self) -> NoteOnBuilder<M> {
        NoteOnBuilder::new()
    }
    pub fn pitch_bend(self) -> PitchBendBuilder<M> {
        PitchBendBuilder::new()
    }
    pub fn program_change(self) -> ProgramChangeBuilder<M> {
        ProgramChangeBuilder::new()
    }
}

impl Midi1ChannelVoiceOwned {
    pub fn builder() -> Midi1ChannelVoiceBuilder<Self> {
        Midi1ChannelVoiceBuilder::new()
    }
}

impl<'a> Midi1ChannelVoiceMessage<'a> {
    pub fn builder() -> Midi1ChannelVoiceBuilder<Self> {
        Midi1ChannelVoiceBuilder::new()
    }
}

const CHANNEL_PRESSURE_CODE: u32 = 0b1101;
const CONTROL_CHANGE_CODE: u32 = 0b1011;
const KEY_PRESSURE_CODE: u32 = 0b1010;
const NOTE_OFF_CODE: u32 = 0b1000;
const NOTE_ON_CODE: u32 = 0b1001;
const PITCH_BEND_CODE: u32 = 0b1110;
const PROGRAM_CHANGE_CODE: u32 = 0b1100;

const CHANNEL_PRESSURE_CODE_U8: u8 = 0b1101;
const CONTROL_CHANGE_CODE_U8: u8 = 0b1011;
const KEY_PRESSURE_CODE_U8: u8 = 0b1010;
const NOTE_OFF_CODE_U8: u8 = 0b1000;
const NOTE_ON_CODE_U8: u8 = 0b1001;
const PITCH_BEND_CODE_U8: u8 = 0b1110;
const PROGRAM_CHANGE_CODE_U8: u8 = 0b1100;

impl<'a> FromData<'a> for Midi1ChannelVoiceBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        use Midi1ChannelVoiceBorrowed::*;
        match u32::from(buffer[0].nibble(2)) {
            CHANNEL_PRESSURE_CODE => {
                ChannelPressure(ChannelPressureBorrowed::from_data_unchecked(buffer))
            }
            CONTROL_CHANGE_CODE => {
                ControlChange(ControlChangeBorrowed::from_data_unchecked(buffer))
            }
            KEY_PRESSURE_CODE => KeyPressure(KeyPressureBorrowed::from_data_unchecked(buffer)),
            NOTE_OFF_CODE => NoteOff(NoteOffBorrowed::from_data_unchecked(buffer)),
            NOTE_ON_CODE => NoteOn(NoteOnBorrowed::from_data_unchecked(buffer)),
            PITCH_BEND_CODE => PitchBend(PitchBendBorrowed::from_data_unchecked(buffer)),
            PROGRAM_CHANGE_CODE => {
                ProgramChange(ProgramChangeBorrowed::from_data_unchecked(buffer))
            }
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u32::from(buffer[0].nibble(2)) {
            CHANNEL_PRESSURE_CODE => ChannelPressureBorrowed::validate_data(buffer),
            CONTROL_CHANGE_CODE => ControlChangeBorrowed::validate_data(buffer),
            KEY_PRESSURE_CODE => KeyPressureBorrowed::validate_data(buffer),
            NOTE_OFF_CODE => NoteOffBorrowed::validate_data(buffer),
            NOTE_ON_CODE => NoteOnBorrowed::validate_data(buffer),
            PITCH_BEND_CODE => PitchBendBorrowed::validate_data(buffer),
            PROGRAM_CHANGE_CODE => ProgramChangeBorrowed::validate_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'a> FromData<'a> for Midi1ChannelVoiceMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Midi1ChannelVoiceBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Midi1ChannelVoiceBorrowed::from_data_unchecked(buffer).into()
    }
}

impl<'a> FromByteData<'a> for Midi1ChannelVoiceOwned {
    type Target = Self;
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        use Midi1ChannelVoiceOwned::*;
        match u8::from(buffer[0].nibble(0)) {
            CHANNEL_PRESSURE_CODE_U8 => {
                ChannelPressure(ChannelPressureOwned::from_byte_data_unchecked(buffer))
            }
            CONTROL_CHANGE_CODE_U8 => {
                ControlChange(ControlChangeOwned::from_byte_data_unchecked(buffer))
            }
            KEY_PRESSURE_CODE_U8 => KeyPressure(KeyPressureOwned::from_byte_data_unchecked(buffer)),
            NOTE_OFF_CODE_U8 => NoteOff(NoteOffOwned::from_byte_data_unchecked(buffer)),
            NOTE_ON_CODE_U8 => NoteOn(NoteOnOwned::from_byte_data_unchecked(buffer)),
            PITCH_BEND_CODE_U8 => PitchBend(PitchBendOwned::from_byte_data_unchecked(buffer)),
            PROGRAM_CHANGE_CODE_U8 => {
                ProgramChange(ProgramChangeOwned::from_byte_data_unchecked(buffer))
            }
            _ => panic!(),
        }
    }
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        if buffer.len() < 3 {
            return Err(Error::BufferOverflow);
        }
        match u8::from(buffer[0].nibble(0)) {
            CHANNEL_PRESSURE_CODE_U8 => ChannelPressureOwned::validate_byte_data(buffer),
            CONTROL_CHANGE_CODE_U8 => ControlChangeOwned::validate_byte_data(buffer),
            KEY_PRESSURE_CODE_U8 => KeyPressureOwned::validate_byte_data(buffer),
            NOTE_OFF_CODE_U8 => NoteOffOwned::validate_byte_data(buffer),
            NOTE_ON_CODE_U8 => NoteOnOwned::validate_byte_data(buffer),
            PITCH_BEND_CODE_U8 => PitchBendOwned::validate_byte_data(buffer),
            PROGRAM_CHANGE_CODE_U8 => ProgramChangeOwned::validate_byte_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'a, 'b> FromByteData<'a> for Midi1ChannelVoiceMessage<'b> {
    type Target = Self;
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        Midi1ChannelVoiceOwned::validate_byte_data(buffer)
    }
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        Midi1ChannelVoiceOwned::from_byte_data_unchecked(buffer).into()
    }
}

impl<'a> core::convert::From<Midi1ChannelVoiceBorrowed<'a>> for Midi1ChannelVoiceMessage<'a> {
    fn from(value: Midi1ChannelVoiceBorrowed<'a>) -> Self {
        use Midi1ChannelVoiceBorrowed as B;
        use Midi1ChannelVoiceMessage as M;
        match value {
            B::ChannelPressure(m) => M::ChannelPressure(m.into()),
            B::ControlChange(m) => M::ControlChange(m.into()),
            B::KeyPressure(m) => M::KeyPressure(m.into()),
            B::NoteOff(m) => M::NoteOff(m.into()),
            B::NoteOn(m) => M::NoteOn(m.into()),
            B::PitchBend(m) => M::PitchBend(m.into()),
            B::ProgramChange(m) => M::ProgramChange(m.into()),
        }
    }
}

impl<'a> core::convert::From<Midi1ChannelVoiceOwned> for Midi1ChannelVoiceMessage<'a> {
    fn from(value: Midi1ChannelVoiceOwned) -> Self {
        use Midi1ChannelVoiceMessage as M;
        use Midi1ChannelVoiceOwned as O;
        match value {
            O::ChannelPressure(m) => M::ChannelPressure(m.into()),
            O::ControlChange(m) => M::ControlChange(m.into()),
            O::KeyPressure(m) => M::KeyPressure(m.into()),
            O::NoteOff(m) => M::NoteOff(m.into()),
            O::NoteOn(m) => M::NoteOn(m.into()),
            O::PitchBend(m) => M::PitchBend(m.into()),
            O::ProgramChange(m) => M::ProgramChange(m.into()),
        }
    }
}

impl<'a> IntoOwned for Midi1ChannelVoiceBorrowed<'a> {
    type Owned = Midi1ChannelVoiceOwned;
    fn into_owned(self) -> Self::Owned {
        use Midi1ChannelVoiceBorrowed as B;
        use Midi1ChannelVoiceOwned as O;
        match self {
            B::ChannelPressure(m) => O::ChannelPressure(m.into_owned()),
            B::ControlChange(m) => O::ControlChange(m.into_owned()),
            B::KeyPressure(m) => O::KeyPressure(m.into_owned()),
            B::NoteOff(m) => O::NoteOff(m.into_owned()),
            B::NoteOn(m) => O::NoteOn(m.into_owned()),
            B::PitchBend(m) => O::PitchBend(m.into_owned()),
            B::ProgramChange(m) => O::ProgramChange(m.into_owned()),
        }
    }
}

impl<'a> IntoOwned for Midi1ChannelVoiceMessage<'a> {
    type Owned = Midi1ChannelVoiceOwned;
    fn into_owned(self) -> Self::Owned {
        use Midi1ChannelVoiceMessage as M;
        use Midi1ChannelVoiceOwned as O;
        match self {
            M::ChannelPressure(m) => O::ChannelPressure(m.into_owned()),
            M::ControlChange(m) => O::ControlChange(m.into_owned()),
            M::KeyPressure(m) => O::KeyPressure(m.into_owned()),
            M::NoteOff(m) => O::NoteOff(m.into_owned()),
            M::NoteOn(m) => O::NoteOn(m.into_owned()),
            M::PitchBend(m) => O::PitchBend(m.into_owned()),
            M::ProgramChange(m) => O::ProgramChange(m.into_owned()),
        }
    }
}

macro_rules! from_message_impl {
    ($message: ty) => {
        impl<'a> core::convert::From<$message> for Midi1ChannelVoiceMessage<'a> {
            fn from(value: $message) -> Self {
                <Midi1ChannelVoiceOwned as core::convert::From<$message>>::from(value).into()
            }
        }
    };
}

from_message_impl!(ChannelPressureOwned);
from_message_impl!(ControlChangeOwned);
from_message_impl!(KeyPressureOwned);
from_message_impl!(NoteOffOwned);
from_message_impl!(NoteOnOwned);
from_message_impl!(PitchBendOwned);
from_message_impl!(ProgramChangeOwned);

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        assert_eq!(
            Midi1ChannelVoiceMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }
}
