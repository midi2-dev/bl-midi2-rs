use crate::{util::BitOps, *};

mod assignable_controller;
mod assignable_per_note_controller;
mod attribute;
mod channel_pitch_bend;
mod channel_pressure;
mod control_change;
mod controller;
mod key_pressure;
mod note_off;
mod note_on;
mod per_note_management;
mod per_note_pitch_bend;
mod program_change;
mod registered_controller;
mod registered_per_note_controller;
mod relative_assignable_controller;
mod relative_registered_controller;

pub use attribute::Attribute as NoteAttribute;
pub use controller::Controller;

pub use assignable_controller::AssignableController;
pub use assignable_controller::AssignableControllerBorrowed;
pub use assignable_controller::AssignableControllerBuilder;
pub use assignable_controller::AssignableControllerOwned;
pub use assignable_per_note_controller::AssignablePerNoteController;
pub use assignable_per_note_controller::AssignablePerNoteControllerBorrowed;
pub use assignable_per_note_controller::AssignablePerNoteControllerBuilder;
pub use assignable_per_note_controller::AssignablePerNoteControllerOwned;
pub use channel_pitch_bend::ChannelPitchBend;
pub use channel_pitch_bend::ChannelPitchBendBorrowed;
pub use channel_pitch_bend::ChannelPitchBendBuilder;
pub use channel_pitch_bend::ChannelPitchBendOwned;
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
pub use per_note_management::PerNoteManagement;
pub use per_note_management::PerNoteManagementBorrowed;
pub use per_note_management::PerNoteManagementBuilder;
pub use per_note_management::PerNoteManagementOwned;
pub use per_note_pitch_bend::PerNotePitchBend;
pub use per_note_pitch_bend::PerNotePitchBendBorrowed;
pub use per_note_pitch_bend::PerNotePitchBendBuilder;
pub use per_note_pitch_bend::PerNotePitchBendOwned;
pub use program_change::ProgramChange;
pub use program_change::ProgramChangeBorrowed;
pub use program_change::ProgramChangeBuilder;
pub use program_change::ProgramChangeOwned;
pub use registered_controller::RegisteredController;
pub use registered_controller::RegisteredControllerBorrowed;
pub use registered_controller::RegisteredControllerBuilder;
pub use registered_controller::RegisteredControllerOwned;
pub use registered_per_note_controller::RegisteredPerNoteController;
pub use registered_per_note_controller::RegisteredPerNoteControllerBorrowed;
pub use registered_per_note_controller::RegisteredPerNoteControllerBuilder;
pub use registered_per_note_controller::RegisteredPerNoteControllerOwned;
pub use relative_assignable_controller::RelativeAssignableController;
pub use relative_assignable_controller::RelativeAssignableControllerBorrowed;
pub use relative_assignable_controller::RelativeAssignableControllerBuilder;
pub use relative_assignable_controller::RelativeAssignableControllerOwned;
pub use relative_registered_controller::RelativeRegisteredController;
pub use relative_registered_controller::RelativeRegisteredControllerBorrowed;
pub use relative_registered_controller::RelativeRegisteredControllerBuilder;
pub use relative_registered_controller::RelativeRegisteredControllerOwned;

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
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

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
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
    pub fn note_off(self) -> NoteOnBuilder<M> {
        NoteOnBuilder::new()
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

impl<'a> Data for Midi2ChannelVoiceBorrowed<'a> {
    fn data(&self) -> &[u32] {
        use Midi2ChannelVoiceBorrowed::*;
        match self {
            AssignableController(m) => m.data(),
            AssignablePerNoteController(m) => m.data(),
            ChannelPitchBend(m) => m.data(),
            ChannelPressure(m) => m.data(),
            ControlChange(m) => m.data(),
            KeyPressure(m) => m.data(),
            NoteOff(m) => m.data(),
            NoteOn(m) => m.data(),
            PerNoteManagement(m) => m.data(),
            PerNotePitchBend(m) => m.data(),
            ProgramChange(m) => m.data(),
            RegisteredController(m) => m.data(),
            RegisteredPerNoteController(m) => m.data(),
            RelativeAssignableController(m) => m.data(),
            RelativeRegisteredController(m) => m.data(),
        }
    }
}

impl Data for Midi2ChannelVoiceOwned {
    fn data(&self) -> &[u32] {
        use Midi2ChannelVoiceOwned::*;
        match self {
            AssignableController(m) => m.data(),
            AssignablePerNoteController(m) => m.data(),
            ChannelPitchBend(m) => m.data(),
            ChannelPressure(m) => m.data(),
            ControlChange(m) => m.data(),
            KeyPressure(m) => m.data(),
            NoteOff(m) => m.data(),
            NoteOn(m) => m.data(),
            PerNoteManagement(m) => m.data(),
            PerNotePitchBend(m) => m.data(),
            ProgramChange(m) => m.data(),
            RegisteredController(m) => m.data(),
            RegisteredPerNoteController(m) => m.data(),
            RelativeAssignableController(m) => m.data(),
            RelativeRegisteredController(m) => m.data(),
        }
    }
}

impl<'a> Grouped for Midi2ChannelVoiceBorrowed<'a> {
    fn group(&self) -> u4 {
        use Midi2ChannelVoiceBorrowed::*;
        match self {
            AssignableController(m) => m.group(),
            AssignablePerNoteController(m) => m.group(),
            ChannelPitchBend(m) => m.group(),
            ChannelPressure(m) => m.group(),
            ControlChange(m) => m.group(),
            KeyPressure(m) => m.group(),
            NoteOff(m) => m.group(),
            NoteOn(m) => m.group(),
            PerNoteManagement(m) => m.group(),
            PerNotePitchBend(m) => m.group(),
            ProgramChange(m) => m.group(),
            RegisteredController(m) => m.group(),
            RegisteredPerNoteController(m) => m.group(),
            RelativeAssignableController(m) => m.group(),
            RelativeRegisteredController(m) => m.group(),
        }
    }
}

impl Grouped for Midi2ChannelVoiceOwned {
    fn group(&self) -> u4 {
        use Midi2ChannelVoiceOwned::*;
        match self {
            AssignableController(m) => m.group(),
            AssignablePerNoteController(m) => m.group(),
            ChannelPitchBend(m) => m.group(),
            ChannelPressure(m) => m.group(),
            ControlChange(m) => m.group(),
            KeyPressure(m) => m.group(),
            NoteOff(m) => m.group(),
            NoteOn(m) => m.group(),
            PerNoteManagement(m) => m.group(),
            PerNotePitchBend(m) => m.group(),
            ProgramChange(m) => m.group(),
            RegisteredController(m) => m.group(),
            RegisteredPerNoteController(m) => m.group(),
            RelativeAssignableController(m) => m.group(),
            RelativeRegisteredController(m) => m.group(),
        }
    }
}

impl<'a> FromData<'a> for Midi2ChannelVoiceBorrowed<'a> {
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
