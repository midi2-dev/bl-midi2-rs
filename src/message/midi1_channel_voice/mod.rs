use crate::{util::BitOps, *};

mod channel_pressure;
mod control_change;
mod key_pressure;
mod note_off;
mod note_on;
mod pitch_bend;
mod program_change;

pub const TYPE_CODE: u32 = 0x2;

pub use channel_pressure::ChannelPressure;
pub use channel_pressure::ChannelPressureBorrowed;
pub use channel_pressure::ChannelPressureBuilder;
pub use channel_pressure::ChannelPressureOwned;
pub use control_change::ControlChange;
pub use control_change::ControlChangeBorrowed;
pub use control_change::ControlChangeBuilder;
pub use control_change::ControlChangeOwned;
pub use key_pressure::KeyPressure;
pub use key_pressure::KeyPressureBorrowed;
pub use key_pressure::KeyPressureBuilder;
pub use key_pressure::KeyPressureOwned;
pub use note_off::NoteOff;
pub use note_off::NoteOffBorrowed;
pub use note_off::NoteOffBuilder;
pub use note_off::NoteOffOwned;
pub use note_on::NoteOn;
pub use note_on::NoteOnBorrowed;
pub use note_on::NoteOnBuilder;
pub use note_on::NoteOnOwned;
pub use pitch_bend::PitchBend;
pub use pitch_bend::PitchBendBorrowed;
pub use pitch_bend::PitchBendBuilder;
pub use pitch_bend::PitchBendOwned;
pub use program_change::ProgramChange;
pub use program_change::ProgramChangeBorrowed;
pub use program_change::ProgramChangeBuilder;
pub use program_change::ProgramChangeOwned;

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
pub enum Midi1ChannelVoiceBorrowed<'a> {
    ChannelPressure(ChannelPressureBorrowed<'a>),
    ControlChange(ControlChangeBorrowed<'a>),
    KeyPressure(KeyPressureBorrowed<'a>),
    NoteOff(NoteOffBorrowed<'a>),
    NoteOn(NoteOnBorrowed<'a>),
    PitchBend(PitchBendBorrowed<'a>),
    ProgramChange(ProgramChangeBorrowed<'a>),
}

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
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

pub const CHANNEL_PRESSURE_CODE: u32 = 0b1101;
pub const CONTROL_CHANGE_CODE: u32 = 0b1011;
pub const KEY_PRESSURE_CODE: u32 = 0b1010;
pub const NOTE_OFF_CODE: u32 = 0b1000;
pub const NOTE_ON_CODE: u32 = 0b1001;
pub const PITCH_BEND_CODE: u32 = 0b1110;
pub const PROGRAM_CHANGE_CODE: u32 = 0b1100;

impl<'a> Data for Midi1ChannelVoiceBorrowed<'a> {
    fn data(&self) -> &[u32] {
        use Midi1ChannelVoiceBorrowed::*;
        match self {
            ChannelPressure(m) => m.data(),
            ControlChange(m) => m.data(),
            KeyPressure(m) => m.data(),
            NoteOff(m) => m.data(),
            NoteOn(m) => m.data(),
            PitchBend(m) => m.data(),
            ProgramChange(m) => m.data(),
        }
    }
}

impl Data for Midi1ChannelVoiceOwned {
    fn data(&self) -> &[u32] {
        use Midi1ChannelVoiceOwned::*;
        match self {
            ChannelPressure(m) => m.data(),
            ControlChange(m) => m.data(),
            KeyPressure(m) => m.data(),
            NoteOff(m) => m.data(),
            NoteOn(m) => m.data(),
            PitchBend(m) => m.data(),
            ProgramChange(m) => m.data(),
        }
    }
}

impl<'a> Grouped for Midi1ChannelVoiceBorrowed<'a> {
    fn group(&self) -> u4 {
        use Midi1ChannelVoiceBorrowed::*;
        match self {
            ChannelPressure(m) => m.group(),
            ControlChange(m) => m.group(),
            KeyPressure(m) => m.group(),
            NoteOff(m) => m.group(),
            NoteOn(m) => m.group(),
            PitchBend(m) => m.group(),
            ProgramChange(m) => m.group(),
        }
    }
}

impl Grouped for Midi1ChannelVoiceOwned {
    fn group(&self) -> u4 {
        use Midi1ChannelVoiceOwned::*;
        match self {
            ChannelPressure(m) => m.group(),
            ControlChange(m) => m.group(),
            KeyPressure(m) => m.group(),
            NoteOff(m) => m.group(),
            NoteOn(m) => m.group(),
            PitchBend(m) => m.group(),
            ProgramChange(m) => m.group(),
        }
    }
}

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

impl<'a> ToOwned for Midi1ChannelVoiceBorrowed<'a> {
    type Owned = Midi1ChannelVoiceOwned;
    fn to_owned(self) -> Self::Owned {
        use Midi1ChannelVoiceBorrowed as B;
        use Midi1ChannelVoiceOwned as O;
        match self {
            B::ChannelPressure(m) => O::ChannelPressure(m.to_owned()),
            B::ControlChange(m) => O::ControlChange(m.to_owned()),
            B::KeyPressure(m) => O::KeyPressure(m.to_owned()),
            B::NoteOff(m) => O::NoteOff(m.to_owned()),
            B::NoteOn(m) => O::NoteOn(m.to_owned()),
            B::PitchBend(m) => O::PitchBend(m.to_owned()),
            B::ProgramChange(m) => O::ProgramChange(m.to_owned()),
        }
    }
}
