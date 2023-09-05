use crate::{util::BitOps, *};

mod assignable_controller;
mod assignable_per_note_controller;
mod attribute;
mod channel_pitch_bend;
mod channel_pressure;
mod control_change;
mod controller;
mod helpers;
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

pub(crate) const TYPE_CODE: u4 = u4::new(0x4);

pub use attribute::Attribute as NoteAttribute;
pub use controller::Controller;

pub use assignable_controller::AssignableControllerBuilder;
pub use assignable_controller::AssignableControllerMessage;
pub use assignable_per_note_controller::AssignablePerNoteControllerBuilder;
pub use assignable_per_note_controller::AssignablePerNoteControllerMessage;
pub use channel_pitch_bend::ChannelPitchBendBuilder;
pub use channel_pitch_bend::ChannelPitchBendMessage;
pub use channel_pressure::ChannelPressureBuilder;
pub use channel_pressure::ChannelPressureMessage;
pub use control_change::ControlChangeBuilder;
pub use control_change::ControlChangeMessage;
pub use key_pressure::KeyPressureBuilder;
pub use key_pressure::KeyPressureMessage;
pub use note_off::NoteOffBuilder;
pub use note_off::NoteOffMessage;
pub use note_on::NoteOnBuilder;
pub use note_on::NoteOnMessage;
pub use per_note_management::PerNoteManagementBuilder;
pub use per_note_management::PerNoteManagementMessage;
pub use per_note_pitch_bend::PerNotePitchBendBuilder;
pub use per_note_pitch_bend::PerNotePitchBendMessage;
pub use program_change::ProgramChangeBuilder;
pub use program_change::ProgramChangeMessage;
pub use registered_controller::RegisteredControllerBuilder;
pub use registered_controller::RegisteredControllerMessage;
pub use registered_per_note_controller::RegisteredPerNoteControllerBuilder;
pub use registered_per_note_controller::RegisteredPerNoteControllerMessage;
pub use relative_assignable_controller::RelativeAssignableControllerBuilder;
pub use relative_assignable_controller::RelativeAssignableControllerMessage;
pub use relative_registered_controller::RelativeRegisteredControllerBuilder;
pub use relative_registered_controller::RelativeRegisteredControllerMessage;

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

use Midi2ChannelVoiceMessage::*;

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

impl<'a> Message<'a, Ump> for Midi2ChannelVoiceMessage<'a> {
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(2)) {
            ASSIGNABLE_CONTROLLER_CODE => AssignableControllerMessage::validate_data(buffer),
            ASSIGNABLE_PER_NOTE_CONTROLLER_CODE => {
                AssignablePerNoteControllerMessage::validate_data(buffer)
            }
            CHANNEL_PITCH_BEND_CODE => ChannelPitchBendMessage::validate_data(buffer),
            CHANNEL_PRESSURE_CODE => ChannelPressureMessage::validate_data(buffer),
            CONTROL_CHANGE_CODE => ControlChangeMessage::validate_data(buffer),
            KEY_PRESSURE_CODE => KeyPressureMessage::validate_data(buffer),
            NOTE_OFF_CODE => NoteOffMessage::validate_data(buffer),
            NOTE_ON_CODE => NoteOnMessage::validate_data(buffer),
            PER_NOTE_MANAGEMENT_CODE => PerNoteManagementMessage::validate_data(buffer),
            PER_NOTE_PITCH_BEND_CODE => PerNotePitchBendMessage::validate_data(buffer),
            PROGRAM_CHANGE_CODE => ProgramChangeMessage::validate_data(buffer),
            REGISTERED_CONTROLLER_CODE => RegisteredControllerMessage::validate_data(buffer),
            REGISTERED_PER_NOTE_CONTROLLER_CODE => {
                RegisteredPerNoteControllerMessage::validate_data(buffer)
            }
            RELATIVE_ASSIGNABLE_CONTROLLER_CODE => {
                RelativeAssignableControllerMessage::validate_data(buffer)
            }
            RELATIVE_REGISTERED_CONTROLLER_CODE => {
                RelativeRegisteredControllerMessage::validate_data(buffer)
            }
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        match u8::from(buffer[0].nibble(2)) {
            ASSIGNABLE_CONTROLLER_CODE => {
                AssignableController(AssignableControllerMessage::from_data_unchecked(buffer))
            }
            ASSIGNABLE_PER_NOTE_CONTROLLER_CODE => AssignablePerNoteController(
                AssignablePerNoteControllerMessage::from_data_unchecked(buffer),
            ),
            CHANNEL_PITCH_BEND_CODE => {
                ChannelPitchBend(ChannelPitchBendMessage::from_data_unchecked(buffer))
            }
            CHANNEL_PRESSURE_CODE => {
                ChannelPressure(ChannelPressureMessage::from_data_unchecked(buffer))
            }
            CONTROL_CHANGE_CODE => ControlChange(ControlChangeMessage::from_data_unchecked(buffer)),
            KEY_PRESSURE_CODE => KeyPressure(KeyPressureMessage::from_data_unchecked(buffer)),
            NOTE_OFF_CODE => NoteOff(NoteOffMessage::from_data_unchecked(buffer)),
            NOTE_ON_CODE => NoteOn(NoteOnMessage::from_data_unchecked(buffer)),
            PER_NOTE_MANAGEMENT_CODE => {
                PerNoteManagement(PerNoteManagementMessage::from_data_unchecked(buffer))
            }
            PER_NOTE_PITCH_BEND_CODE => {
                PerNotePitchBend(PerNotePitchBendMessage::from_data_unchecked(buffer))
            }
            PROGRAM_CHANGE_CODE => ProgramChange(ProgramChangeMessage::from_data_unchecked(buffer)),
            REGISTERED_CONTROLLER_CODE => {
                RegisteredController(RegisteredControllerMessage::from_data_unchecked(buffer))
            }
            REGISTERED_PER_NOTE_CONTROLLER_CODE => RegisteredPerNoteController(
                RegisteredPerNoteControllerMessage::from_data_unchecked(buffer),
            ),
            RELATIVE_ASSIGNABLE_CONTROLLER_CODE => RelativeAssignableController(
                RelativeAssignableControllerMessage::from_data_unchecked(buffer),
            ),
            RELATIVE_REGISTERED_CONTROLLER_CODE => RelativeRegisteredController(
                RelativeRegisteredControllerMessage::from_data_unchecked(buffer),
            ),
            _ => panic!(),
        }
    }
    fn data(&self) -> &'a [u32] {
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
