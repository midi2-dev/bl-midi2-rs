// use crate::{
//     util::{schema::*, BitOps},
//     *,
// };

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

// pub enum Midi1ChannelVoiceMessage<'a, B>
// where
//     B: Buffer
//         + Property<
//             NumericalConstant<CHANNEL_PRESSURE_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<CONTROL_CHANGE_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<KEY_PRESSURE_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<NOTE_OFF_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<NOTE_ON_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<PITCH_BEND_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<PROGRAM_CHANGE_CODE>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<NumericalConstant<TYPE_CODE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>
//         + Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x7F>>
//         + Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, BytesSchema<0x0F, 0x0, 0x0>>
//         + Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>
//         + Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
// {
//     ChannelPressure(ChannelPressureMessage<'a, B>),
//     ControlChange(ControlChangeMessage<'a, B>),
//     KeyPressure(KeyPressureMessage<'a, B>),
//     NoteOff(NoteOffMessage<'a, B>),
//     NoteOn(NoteOnMessage<'a, B>),
//     PitchBend(PitchBendMessage<'a, B>),
//     ProgramChange(ProgramChangeMessage<'a, B>),
// }
//
// pub const CHANNEL_PRESSURE_CODE: u32 = 0b1101;
// pub const CONTROL_CHANGE_CODE: u32 = 0b1011;
// pub const KEY_PRESSURE_CODE: u32 = 0b1010;
// pub const NOTE_OFF_CODE: u32 = 0b1000;
// pub const NOTE_ON_CODE: u32 = 0b1001;
// pub const PITCH_BEND_CODE: u32 = 0b1110;
// pub const PROGRAM_CHANGE_CODE: u32 = 0b1100;
//
// use Midi1ChannelVoiceMessage::*;
//
// impl<'a> Message<'a, Ump> for Midi1ChannelVoiceMessage<'a, Ump> {
//     fn data(&self) -> &'a [u32] {
//         match self {
//             ChannelPressure(m) => m.data(),
//             ControlChange(m) => m.data(),
//             KeyPressure(m) => m.data(),
//             NoteOff(m) => m.data(),
//             NoteOn(m) => m.data(),
//             PitchBend(m) => m.data(),
//             ProgramChange(m) => m.data(),
//         }
//     }
//     fn from_data_unchecked(buffer: &'a [u32]) -> Self {
//         match u32::from(buffer[0].nibble(2)) {
//             CHANNEL_PRESSURE_CODE => {
//                 ChannelPressure(ChannelPressureMessage::from_data_unchecked(buffer))
//             }
//             CONTROL_CHANGE_CODE => ControlChange(ControlChangeMessage::from_data_unchecked(buffer)),
//             KEY_PRESSURE_CODE => KeyPressure(KeyPressureMessage::from_data_unchecked(buffer)),
//             NOTE_OFF_CODE => NoteOff(NoteOffMessage::from_data_unchecked(buffer)),
//             NOTE_ON_CODE => NoteOn(NoteOnMessage::from_data_unchecked(buffer)),
//             PITCH_BEND_CODE => PitchBend(PitchBendMessage::from_data_unchecked(buffer)),
//             PROGRAM_CHANGE_CODE => ProgramChange(ProgramChangeMessage::from_data_unchecked(buffer)),
//             _ => panic!(),
//         }
//     }
//     fn validate_data(buffer: &'a [u32]) -> Result<()> {
//         match u32::from(buffer[0].nibble(2)) {
//             CHANNEL_PRESSURE_CODE => ChannelPressureMessage::<Ump>::validate_data(buffer),
//             CONTROL_CHANGE_CODE => ControlChangeMessage::<Ump>::validate_data(buffer),
//             KEY_PRESSURE_CODE => KeyPressureMessage::<Ump>::validate_data(buffer),
//             NOTE_OFF_CODE => NoteOffMessage::<Ump>::validate_data(buffer),
//             NOTE_ON_CODE => NoteOnMessage::<Ump>::validate_data(buffer),
//             PITCH_BEND_CODE => PitchBendMessage::<Ump>::validate_data(buffer),
//             PROGRAM_CHANGE_CODE => ProgramChangeMessage::<Ump>::validate_data(buffer),
//             _ => Err(Error::InvalidData),
//         }
//     }
// }
