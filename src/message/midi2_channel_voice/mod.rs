use crate::{util::BitOps, *};

pub mod assignable_controller;
pub mod assignable_per_note_controller;
pub mod attribute;
pub mod channel_pitch_bend;
pub mod channel_pressure;
pub mod control_change;
pub mod controller;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod per_note_management;
pub mod per_note_pitch_bend;
pub mod program_change;
pub mod registered_controller;
pub mod registered_per_note_controller;
pub mod relative_assignable_controller;
pub mod relative_registered_controller;

pub use attribute::Attribute as NoteAttribute;
pub use controller::Controller;

use assignable_controller::AssignableControllerBorrowed;
use assignable_controller::AssignableControllerBuilder;
use assignable_controller::AssignableControllerMessage;
use assignable_controller::AssignableControllerOwned;
use assignable_per_note_controller::AssignablePerNoteControllerBorrowed;
use assignable_per_note_controller::AssignablePerNoteControllerBuilder;
use assignable_per_note_controller::AssignablePerNoteControllerMessage;
use assignable_per_note_controller::AssignablePerNoteControllerOwned;
use channel_pitch_bend::ChannelPitchBendBorrowed;
use channel_pitch_bend::ChannelPitchBendBuilder;
use channel_pitch_bend::ChannelPitchBendMessage;
use channel_pitch_bend::ChannelPitchBendOwned;
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
use per_note_management::PerNoteManagementBorrowed;
use per_note_management::PerNoteManagementBuilder;
use per_note_management::PerNoteManagementMessage;
use per_note_management::PerNoteManagementOwned;
use per_note_pitch_bend::PerNotePitchBendBorrowed;
use per_note_pitch_bend::PerNotePitchBendBuilder;
use per_note_pitch_bend::PerNotePitchBendMessage;
use per_note_pitch_bend::PerNotePitchBendOwned;
use program_change::ProgramChangeBorrowed;
use program_change::ProgramChangeBuilder;
use program_change::ProgramChangeMessage;
use program_change::ProgramChangeOwned;
use registered_controller::RegisteredControllerBorrowed;
use registered_controller::RegisteredControllerBuilder;
use registered_controller::RegisteredControllerMessage;
use registered_controller::RegisteredControllerOwned;
use registered_per_note_controller::RegisteredPerNoteControllerBorrowed;
use registered_per_note_controller::RegisteredPerNoteControllerBuilder;
use registered_per_note_controller::RegisteredPerNoteControllerMessage;
use registered_per_note_controller::RegisteredPerNoteControllerOwned;
use relative_assignable_controller::RelativeAssignableControllerBorrowed;
use relative_assignable_controller::RelativeAssignableControllerBuilder;
use relative_assignable_controller::RelativeAssignableControllerMessage;
use relative_assignable_controller::RelativeAssignableControllerOwned;
use relative_registered_controller::RelativeRegisteredControllerBorrowed;
use relative_registered_controller::RelativeRegisteredControllerBuilder;
use relative_registered_controller::RelativeRegisteredControllerMessage;
use relative_registered_controller::RelativeRegisteredControllerOwned;

#[derive(
    derive_more::From,
    midi2_attr::Channeled,
    midi2_attr::Data,
    midi2_attr::Grouped,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi2ChannelVoiceMessage<'a> {
    AssignableController(AssignableControllerMessage<'a>),
    AssignablePerNoteController(AssignablePerNoteControllerMessage<'a>),
    ChannelPitchBend(ChannelPitchBendMessage<'a>),
    ChannelPressure(ChannelPressureMessage<'a>),
    ControlChange(ControlChangeMessage<'a>),
    KeyPressure(KeyPressureMessage<'a>),
    NoteOff(NoteOffMessage<'a>),
    NoteOn(NoteOnMessage<'a>),
    PerNoteManagement(PerNoteManagementMessage<'a>),
    PerNotePitchBend(PerNotePitchBendMessage<'a>),
    ProgramChange(ProgramChangeMessage<'a>),
    RegisteredController(RegisteredControllerMessage<'a>),
    RegisteredPerNoteController(RegisteredPerNoteControllerMessage<'a>),
    RelativeAssignableController(RelativeAssignableControllerMessage<'a>),
    RelativeRegisteredController(RelativeRegisteredControllerMessage<'a>),
}

#[derive(
    derive_more::From,
    midi2_attr::Channeled,
    midi2_attr::Data,
    midi2_attr::Grouped,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi2ChannelVoiceBorrowed<'a> {
    AssignableController(AssignableControllerBorrowed<'a>),
    AssignablePerNoteController(AssignablePerNoteControllerBorrowed<'a>),
    ChannelPitchBend(ChannelPitchBendBorrowed<'a>),
    ChannelPressure(ChannelPressureBorrowed<'a>),
    ControlChange(ControlChangeBorrowed<'a>),
    KeyPressure(KeyPressureBorrowed<'a>),
    NoteOff(NoteOffBorrowed<'a>),
    NoteOn(NoteOnBorrowed<'a>),
    PerNoteManagement(PerNoteManagementBorrowed<'a>),
    PerNotePitchBend(PerNotePitchBendBorrowed<'a>),
    ProgramChange(ProgramChangeBorrowed<'a>),
    RegisteredController(RegisteredControllerBorrowed<'a>),
    RegisteredPerNoteController(RegisteredPerNoteControllerBorrowed<'a>),
    RelativeAssignableController(RelativeAssignableControllerBorrowed<'a>),
    RelativeRegisteredController(RelativeRegisteredControllerBorrowed<'a>),
}

#[derive(
    derive_more::From,
    midi2_attr::Channeled,
    midi2_attr::Data,
    midi2_attr::Grouped,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum Midi2ChannelVoiceOwned {
    AssignableController(AssignableControllerOwned),
    AssignablePerNoteController(AssignablePerNoteControllerOwned),
    ChannelPitchBend(ChannelPitchBendOwned),
    ChannelPressure(ChannelPressureOwned),
    ControlChange(ControlChangeOwned),
    KeyPressure(KeyPressureOwned),
    NoteOff(NoteOffOwned),
    NoteOn(NoteOnOwned),
    PerNoteManagement(PerNoteManagementOwned),
    PerNotePitchBend(PerNotePitchBendOwned),
    ProgramChange(ProgramChangeOwned),
    RegisteredController(RegisteredControllerOwned),
    RegisteredPerNoteController(RegisteredPerNoteControllerOwned),
    RelativeAssignableController(RelativeAssignableControllerOwned),
    RelativeRegisteredController(RelativeRegisteredControllerOwned),
}

#[derive(Default)]
pub struct Midi2ChannelVoiceBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<AssignableControllerOwned>
        + core::convert::From<AssignablePerNoteControllerOwned>
        + core::convert::From<ChannelPitchBendOwned>
        + core::convert::From<ChannelPressureOwned>
        + core::convert::From<ControlChangeOwned>
        + core::convert::From<KeyPressureOwned>
        + core::convert::From<NoteOffOwned>
        + core::convert::From<NoteOnOwned>
        + core::convert::From<PerNoteManagementOwned>
        + core::convert::From<PerNotePitchBendOwned>
        + core::convert::From<ProgramChangeOwned>
        + core::convert::From<RegisteredControllerOwned>
        + core::convert::From<RegisteredPerNoteControllerOwned>
        + core::convert::From<RelativeAssignableControllerOwned>
        + core::convert::From<RelativeRegisteredControllerOwned>;

impl<M> Midi2ChannelVoiceBuilder<M>
where
    M: core::convert::From<AssignableControllerOwned>
        + core::convert::From<AssignablePerNoteControllerOwned>
        + core::convert::From<ChannelPitchBendOwned>
        + core::convert::From<ChannelPressureOwned>
        + core::convert::From<ControlChangeOwned>
        + core::convert::From<KeyPressureOwned>
        + core::convert::From<NoteOffOwned>
        + core::convert::From<NoteOnOwned>
        + core::convert::From<PerNoteManagementOwned>
        + core::convert::From<PerNotePitchBendOwned>
        + core::convert::From<ProgramChangeOwned>
        + core::convert::From<RegisteredControllerOwned>
        + core::convert::From<RegisteredPerNoteControllerOwned>
        + core::convert::From<RelativeAssignableControllerOwned>
        + core::convert::From<RelativeRegisteredControllerOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn assignable_controller(self) -> AssignableControllerBuilder<M> {
        AssignableControllerBuilder::new()
    }
    pub fn assignable_per_note_controller(self) -> AssignablePerNoteControllerBuilder<M> {
        AssignablePerNoteControllerBuilder::new()
    }
    pub fn channel_pitch_bend(self) -> ChannelPitchBendBuilder<M> {
        ChannelPitchBendBuilder::new()
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
    pub fn per_note_management(self) -> PerNoteManagementBuilder<M> {
        PerNoteManagementBuilder::new()
    }
    pub fn per_note_pitch_bend(self) -> PerNotePitchBendBuilder<M> {
        PerNotePitchBendBuilder::new()
    }
    pub fn program_change(self) -> ProgramChangeBuilder<M> {
        ProgramChangeBuilder::new()
    }
    pub fn registered_controller(self) -> RegisteredControllerBuilder<M> {
        RegisteredControllerBuilder::new()
    }
    pub fn registered_per_note_controller(self) -> RegisteredPerNoteControllerBuilder<M> {
        RegisteredPerNoteControllerBuilder::new()
    }
    pub fn relative_assignable_controller(self) -> RelativeAssignableControllerBuilder<M> {
        RelativeAssignableControllerBuilder::new()
    }
    pub fn relative_registered_controller(self) -> RelativeRegisteredControllerBuilder<M> {
        RelativeRegisteredControllerBuilder::new()
    }
}

impl Midi2ChannelVoiceOwned {
    pub fn builder() -> Midi2ChannelVoiceBuilder<Self> {
        Midi2ChannelVoiceBuilder::new()
    }
}

impl<'a> Midi2ChannelVoiceMessage<'a> {
    pub fn builder() -> Midi2ChannelVoiceBuilder<Self> {
        Midi2ChannelVoiceBuilder::new()
    }
}

const ASSIGNABLE_CONTROLLER_CODE: u8 = 0b0011;
const ASSIGNABLE_PER_NOTE_CONTROLLER_CODE: u8 = 0b0001;
const CHANNEL_PITCH_BEND_CODE: u8 = 0b1110;
const CHANNEL_PRESSURE_CODE: u8 = 0b1101;
const CONTROL_CHANGE_CODE: u8 = 0b1011;
const KEY_PRESSURE_CODE: u8 = 0b1010;
const NOTE_OFF_CODE: u8 = 0b1000;
const NOTE_ON_CODE: u8 = 0b1001;
const PER_NOTE_MANAGEMENT_CODE: u8 = 0b1111;
const PER_NOTE_PITCH_BEND_CODE: u8 = 0b0110;
const PROGRAM_CHANGE_CODE: u8 = 0b1100;
const REGISTERED_CONTROLLER_CODE: u8 = 0b0010;
const REGISTERED_PER_NOTE_CONTROLLER_CODE: u8 = 0b0000;
const RELATIVE_ASSIGNABLE_CONTROLLER_CODE: u8 = 0b0101;
const RELATIVE_REGISTERED_CONTROLLER_CODE: u8 = 0b0100;

impl<'a> FromData<'a> for Midi2ChannelVoiceBorrowed<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(2)) {
            ASSIGNABLE_CONTROLLER_CODE => AssignableControllerBorrowed::validate_data(buffer),
            ASSIGNABLE_PER_NOTE_CONTROLLER_CODE => {
                AssignablePerNoteControllerBorrowed::validate_data(buffer)
            }
            CHANNEL_PITCH_BEND_CODE => ChannelPitchBendBorrowed::validate_data(buffer),
            CHANNEL_PRESSURE_CODE => ChannelPressureBorrowed::validate_data(buffer),
            CONTROL_CHANGE_CODE => ControlChangeBorrowed::validate_data(buffer),
            KEY_PRESSURE_CODE => KeyPressureBorrowed::validate_data(buffer),
            NOTE_OFF_CODE => NoteOffBorrowed::validate_data(buffer),
            NOTE_ON_CODE => NoteOnBorrowed::validate_data(buffer),
            PER_NOTE_MANAGEMENT_CODE => PerNoteManagementBorrowed::validate_data(buffer),
            PER_NOTE_PITCH_BEND_CODE => PerNotePitchBendBorrowed::validate_data(buffer),
            PROGRAM_CHANGE_CODE => ProgramChangeBorrowed::validate_data(buffer),
            REGISTERED_CONTROLLER_CODE => RegisteredControllerBorrowed::validate_data(buffer),
            REGISTERED_PER_NOTE_CONTROLLER_CODE => {
                RegisteredPerNoteControllerBorrowed::validate_data(buffer)
            }
            RELATIVE_ASSIGNABLE_CONTROLLER_CODE => {
                RelativeAssignableControllerBorrowed::validate_data(buffer)
            }
            RELATIVE_REGISTERED_CONTROLLER_CODE => {
                RelativeRegisteredControllerBorrowed::validate_data(buffer)
            }
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        use Midi2ChannelVoiceBorrowed::*;
        match u8::from(buffer[0].nibble(2)) {
            ASSIGNABLE_CONTROLLER_CODE => {
                AssignableController(AssignableControllerBorrowed::from_data_unchecked(buffer))
            }
            ASSIGNABLE_PER_NOTE_CONTROLLER_CODE => AssignablePerNoteController(
                AssignablePerNoteControllerBorrowed::from_data_unchecked(buffer),
            ),
            CHANNEL_PITCH_BEND_CODE => {
                ChannelPitchBend(ChannelPitchBendBorrowed::from_data_unchecked(buffer))
            }
            CHANNEL_PRESSURE_CODE => {
                ChannelPressure(ChannelPressureBorrowed::from_data_unchecked(buffer))
            }
            CONTROL_CHANGE_CODE => {
                ControlChange(ControlChangeBorrowed::from_data_unchecked(buffer))
            }
            KEY_PRESSURE_CODE => KeyPressure(KeyPressureBorrowed::from_data_unchecked(buffer)),
            NOTE_OFF_CODE => NoteOff(NoteOffBorrowed::from_data_unchecked(buffer)),
            NOTE_ON_CODE => NoteOn(NoteOnBorrowed::from_data_unchecked(buffer)),
            PER_NOTE_MANAGEMENT_CODE => {
                PerNoteManagement(PerNoteManagementBorrowed::from_data_unchecked(buffer))
            }
            PER_NOTE_PITCH_BEND_CODE => {
                PerNotePitchBend(PerNotePitchBendBorrowed::from_data_unchecked(buffer))
            }
            PROGRAM_CHANGE_CODE => {
                ProgramChange(ProgramChangeBorrowed::from_data_unchecked(buffer))
            }
            REGISTERED_CONTROLLER_CODE => {
                RegisteredController(RegisteredControllerBorrowed::from_data_unchecked(buffer))
            }
            REGISTERED_PER_NOTE_CONTROLLER_CODE => RegisteredPerNoteController(
                RegisteredPerNoteControllerBorrowed::from_data_unchecked(buffer),
            ),
            RELATIVE_ASSIGNABLE_CONTROLLER_CODE => RelativeAssignableController(
                RelativeAssignableControllerBorrowed::from_data_unchecked(buffer),
            ),
            RELATIVE_REGISTERED_CONTROLLER_CODE => RelativeRegisteredController(
                RelativeRegisteredControllerBorrowed::from_data_unchecked(buffer),
            ),
            _ => panic!(),
        }
    }
}

impl<'a> FromData<'a> for Midi2ChannelVoiceMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        Midi2ChannelVoiceBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        Midi2ChannelVoiceBorrowed::from_data_unchecked(buffer).into()
    }
}

impl<'a> IntoOwned for Midi2ChannelVoiceBorrowed<'a> {
    type Owned = Midi2ChannelVoiceOwned;
    fn into_owned(self) -> Self::Owned {
        use Midi2ChannelVoiceBorrowed as B;
        use Midi2ChannelVoiceOwned as O;
        match self {
            B::AssignableController(m) => O::AssignableController(m.into_owned()),
            B::AssignablePerNoteController(m) => O::AssignablePerNoteController(m.into_owned()),
            B::ChannelPitchBend(m) => O::ChannelPitchBend(m.into_owned()),
            B::ChannelPressure(m) => O::ChannelPressure(m.into_owned()),
            B::ControlChange(m) => O::ControlChange(m.into_owned()),
            B::KeyPressure(m) => O::KeyPressure(m.into_owned()),
            B::NoteOff(m) => O::NoteOff(m.into_owned()),
            B::NoteOn(m) => O::NoteOn(m.into_owned()),
            B::PerNoteManagement(m) => O::PerNoteManagement(m.into_owned()),
            B::PerNotePitchBend(m) => O::PerNotePitchBend(m.into_owned()),
            B::ProgramChange(m) => O::ProgramChange(m.into_owned()),
            B::RegisteredController(m) => O::RegisteredController(m.into_owned()),
            B::RegisteredPerNoteController(m) => O::RegisteredPerNoteController(m.into_owned()),
            B::RelativeAssignableController(m) => O::RelativeAssignableController(m.into_owned()),
            B::RelativeRegisteredController(m) => O::RelativeRegisteredController(m.into_owned()),
        }
    }
}

impl<'a> IntoOwned for Midi2ChannelVoiceMessage<'a> {
    type Owned = Midi2ChannelVoiceOwned;
    fn into_owned(self) -> Self::Owned {
        use Midi2ChannelVoiceMessage as M;
        use Midi2ChannelVoiceOwned as O;
        match self {
            M::AssignableController(m) => O::AssignableController(m.into_owned()),
            M::AssignablePerNoteController(m) => O::AssignablePerNoteController(m.into_owned()),
            M::ChannelPitchBend(m) => O::ChannelPitchBend(m.into_owned()),
            M::ChannelPressure(m) => O::ChannelPressure(m.into_owned()),
            M::ControlChange(m) => O::ControlChange(m.into_owned()),
            M::KeyPressure(m) => O::KeyPressure(m.into_owned()),
            M::NoteOff(m) => O::NoteOff(m.into_owned()),
            M::NoteOn(m) => O::NoteOn(m.into_owned()),
            M::PerNoteManagement(m) => O::PerNoteManagement(m.into_owned()),
            M::PerNotePitchBend(m) => O::PerNotePitchBend(m.into_owned()),
            M::ProgramChange(m) => O::ProgramChange(m.into_owned()),
            M::RegisteredController(m) => O::RegisteredController(m.into_owned()),
            M::RegisteredPerNoteController(m) => O::RegisteredPerNoteController(m.into_owned()),
            M::RelativeAssignableController(m) => O::RelativeAssignableController(m.into_owned()),
            M::RelativeRegisteredController(m) => O::RelativeRegisteredController(m.into_owned()),
        }
    }
}

impl<'a> core::convert::From<Midi2ChannelVoiceBorrowed<'a>> for Midi2ChannelVoiceMessage<'a> {
    fn from(value: Midi2ChannelVoiceBorrowed<'a>) -> Self {
        use Midi2ChannelVoiceBorrowed as B;
        use Midi2ChannelVoiceMessage as M;
        match value {
            B::AssignableController(m) => M::AssignableController(m.into()),
            B::AssignablePerNoteController(m) => M::AssignablePerNoteController(m.into()),
            B::ChannelPitchBend(m) => M::ChannelPitchBend(m.into()),
            B::ChannelPressure(m) => M::ChannelPressure(m.into()),
            B::ControlChange(m) => M::ControlChange(m.into()),
            B::KeyPressure(m) => M::KeyPressure(m.into()),
            B::NoteOff(m) => M::NoteOff(m.into()),
            B::NoteOn(m) => M::NoteOn(m.into()),
            B::PerNoteManagement(m) => M::PerNoteManagement(m.into()),
            B::PerNotePitchBend(m) => M::PerNotePitchBend(m.into()),
            B::ProgramChange(m) => M::ProgramChange(m.into()),
            B::RegisteredController(m) => M::RegisteredController(m.into()),
            B::RegisteredPerNoteController(m) => M::RegisteredPerNoteController(m.into()),
            B::RelativeAssignableController(m) => M::RelativeAssignableController(m.into()),
            B::RelativeRegisteredController(m) => M::RelativeRegisteredController(m.into()),
        }
    }
}

impl<'a> core::convert::From<Midi2ChannelVoiceOwned> for Midi2ChannelVoiceMessage<'a> {
    fn from(value: Midi2ChannelVoiceOwned) -> Self {
        use Midi2ChannelVoiceMessage as M;
        use Midi2ChannelVoiceOwned as O;
        match value {
            O::AssignableController(m) => M::AssignableController(m.into()),
            O::AssignablePerNoteController(m) => M::AssignablePerNoteController(m.into()),
            O::ChannelPitchBend(m) => M::ChannelPitchBend(m.into()),
            O::ChannelPressure(m) => M::ChannelPressure(m.into()),
            O::ControlChange(m) => M::ControlChange(m.into()),
            O::KeyPressure(m) => M::KeyPressure(m.into()),
            O::NoteOff(m) => M::NoteOff(m.into()),
            O::NoteOn(m) => M::NoteOn(m.into()),
            O::PerNoteManagement(m) => M::PerNoteManagement(m.into()),
            O::PerNotePitchBend(m) => M::PerNotePitchBend(m.into()),
            O::ProgramChange(m) => M::ProgramChange(m.into()),
            O::RegisteredController(m) => M::RegisteredController(m.into()),
            O::RegisteredPerNoteController(m) => M::RegisteredPerNoteController(m.into()),
            O::RelativeAssignableController(m) => M::RelativeAssignableController(m.into()),
            O::RelativeRegisteredController(m) => M::RelativeRegisteredController(m.into()),
        }
    }
}

macro_rules! from_message_impl {
    ($message: ty) => {
        impl<'a> core::convert::From<$message> for Midi2ChannelVoiceMessage<'a> {
            fn from(value: $message) -> Self {
                <Midi2ChannelVoiceOwned as core::convert::From<$message>>::from(value).into()
            }
        }
    };
}

from_message_impl!(AssignableControllerOwned);
from_message_impl!(AssignablePerNoteControllerOwned);
from_message_impl!(ChannelPitchBendOwned);
from_message_impl!(ChannelPressureOwned);
from_message_impl!(ControlChangeOwned);
from_message_impl!(KeyPressureOwned);
from_message_impl!(NoteOffOwned);
from_message_impl!(NoteOnOwned);
from_message_impl!(PerNoteManagementOwned);
from_message_impl!(PerNotePitchBendOwned);
from_message_impl!(ProgramChangeOwned);
from_message_impl!(RegisteredControllerOwned);
from_message_impl!(RegisteredPerNoteControllerOwned);
from_message_impl!(RelativeAssignableControllerOwned);
from_message_impl!(RelativeRegisteredControllerOwned);

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        assert_eq!(
            Midi2ChannelVoiceMessage::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
                .unwrap()
                .channel(),
            u4::new(0xC),
        );
    }
}
