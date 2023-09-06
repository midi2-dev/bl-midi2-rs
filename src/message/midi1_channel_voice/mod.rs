use crate::{util::BitOps, *};

mod channel_pressure;
mod control_change;
mod helpers;
mod key_pressure;
mod note_off;
mod note_on;
mod pitch_bend;
mod program_change;

const TYPE_CODE: u4 = u4::new(0x2);

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
pub use pitch_bend::PitchBendBuilder;
pub use pitch_bend::PitchBendMessage;
pub use program_change::ProgramChangeBuilder;
pub use program_change::ProgramChangeMessage;

pub enum Midi1ChannelVoiceMessage<'a> {
    ChannelPressure(ChannelPressureMessage<'a>),
    ControlChange(ControlChangeMessage<'a>),
    KeyPressure(KeyPressureMessage<'a>),
    NoteOff(NoteOffMessage<'a>),
    NoteOn(NoteOnMessage<'a>),
    PitchBend(PitchBendMessage<'a>),
    ProgramChange(ProgramChangeMessage<'a>),
}

const CHANNEL_PRESSURE_CODE: u8 = 0b1101;
const CONTROL_CHANGE_CODE: u8 = 0b1011;
const KEY_PRESSURE_CODE: u8 = 0b1010;
const NOTE_OFF_CODE: u8 = 0b1000;
const NOTE_ON_CODE: u8 = 0b1001;
const PITCH_BEND_CODE: u8 = 0b1110;
const PROGRAM_CHANGE_CODE: u8 = 0b1100;

use Midi1ChannelVoiceMessage::*;

impl<'a> Message<'a> for Midi1ChannelVoiceMessage<'a> {
    fn data(&self) -> &'a [u32] {
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
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        match u8::from(buffer[0].nibble(2)) {
            CHANNEL_PRESSURE_CODE => {
                ChannelPressure(ChannelPressureMessage::from_data_unchecked(buffer))
            }
            CONTROL_CHANGE_CODE => ControlChange(ControlChangeMessage::from_data_unchecked(buffer)),
            KEY_PRESSURE_CODE => KeyPressure(KeyPressureMessage::from_data_unchecked(buffer)),
            NOTE_OFF_CODE => NoteOff(NoteOffMessage::from_data_unchecked(buffer)),
            NOTE_ON_CODE => NoteOn(NoteOnMessage::from_data_unchecked(buffer)),
            PITCH_BEND_CODE => PitchBend(PitchBendMessage::from_data_unchecked(buffer)),
            PROGRAM_CHANGE_CODE => ProgramChange(ProgramChangeMessage::from_data_unchecked(buffer)),
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(2)) {
            CHANNEL_PRESSURE_CODE => ChannelPressureMessage::validate_data(buffer),
            CONTROL_CHANGE_CODE => ControlChangeMessage::validate_data(buffer),
            KEY_PRESSURE_CODE => KeyPressureMessage::validate_data(buffer),
            NOTE_OFF_CODE => NoteOffMessage::validate_data(buffer),
            NOTE_ON_CODE => NoteOnMessage::validate_data(buffer),
            PITCH_BEND_CODE => PitchBendMessage::validate_data(buffer),
            PROGRAM_CHANGE_CODE => ProgramChangeMessage::validate_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}
