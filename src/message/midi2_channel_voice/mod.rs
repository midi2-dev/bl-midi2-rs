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
// pub mod per_note_management;
// pub mod per_note_pitch_bend;
// pub mod program_change;
// pub mod registered_controller;
// pub mod registered_per_note_controller;
// pub mod relative_assignable_controller;
// pub mod relative_registered_controller;

pub use attribute::Attribute as NoteAttribute;
pub use controller::Controller;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x4;

// #[derive(
//     derive_more::From,
//     midi2_proc::Channeled,
//     midi2_proc::Data,
//     midi2_proc::Grouped,
//     midi2_proc::UmpDebug,
//     Clone,
//     PartialEq,
//     Eq,
// )]
// pub enum Midi2ChannelVoiceMessage<'a> {
//     AssignableController(AssignableControllerMessage<'a>),
//     AssignablePerNoteController(AssignablePerNoteControllerMessage<'a>),
//     ChannelPitchBend(ChannelPitchBendMessage<'a>),
//     ChannelPressure(ChannelPressureMessage<'a>),
//     ControlChange(ControlChangeMessage<'a>),
//     KeyPressure(KeyPressureMessage<'a>),
//     NoteOff(NoteOffMessage<'a>),
//     NoteOn(NoteOnMessage<'a>),
//     PerNoteManagement(PerNoteManagementMessage<'a>),
//     PerNotePitchBend(PerNotePitchBendMessage<'a>),
//     ProgramChange(ProgramChangeMessage<'a>),
//     RegisteredController(RegisteredControllerMessage<'a>),
//     RegisteredPerNoteController(RegisteredPerNoteControllerMessage<'a>),
//     RelativeAssignableController(RelativeAssignableControllerMessage<'a>),
//     RelativeRegisteredController(RelativeRegisteredControllerMessage<'a>),
// }

// impl<'a> FromData<'a> for Midi2ChannelVoiceBorrowed<'a> {
//     type Target = Self;
//     fn from_data_unchecked(buffer: &'a [u32]) -> Self {
//         use Midi2ChannelVoiceBorrowed::*;
//         match u8::from(buffer[0].nibble(2)) {
//             ASSIGNABLE_CONTROLLER_CODE => {
//                 AssignableController(AssignableControllerBorrowed::from_data_unchecked(buffer))
//             }
//             ASSIGNABLE_PER_NOTE_CONTROLLER_CODE => AssignablePerNoteController(
//                 AssignablePerNoteControllerBorrowed::from_data_unchecked(buffer),
//             ),
//             CHANNEL_PITCH_BEND_CODE => {
//                 ChannelPitchBend(ChannelPitchBendBorrowed::from_data_unchecked(buffer))
//             }
//             CHANNEL_PRESSURE_CODE => {
//                 ChannelPressure(ChannelPressureBorrowed::from_data_unchecked(buffer))
//             }
//             CONTROL_CHANGE_CODE => {
//                 ControlChange(ControlChangeBorrowed::from_data_unchecked(buffer))
//             }
//             KEY_PRESSURE_CODE => KeyPressure(KeyPressureBorrowed::from_data_unchecked(buffer)),
//             NOTE_OFF_CODE => NoteOff(NoteOffBorrowed::from_data_unchecked(buffer)),
//             NOTE_ON_CODE => NoteOn(NoteOnBorrowed::from_data_unchecked(buffer)),
//             PER_NOTE_MANAGEMENT_CODE => {
//                 PerNoteManagement(PerNoteManagementBorrowed::from_data_unchecked(buffer))
//             }
//             PER_NOTE_PITCH_BEND_CODE => {
//                 PerNotePitchBend(PerNotePitchBendBorrowed::from_data_unchecked(buffer))
//             }
//             PROGRAM_CHANGE_CODE => {
//                 ProgramChange(ProgramChangeBorrowed::from_data_unchecked(buffer))
//             }
//             REGISTERED_CONTROLLER_CODE => {
//                 RegisteredController(RegisteredControllerBorrowed::from_data_unchecked(buffer))
//             }
//             REGISTERED_PER_NOTE_CONTROLLER_CODE => RegisteredPerNoteController(
//                 RegisteredPerNoteControllerBorrowed::from_data_unchecked(buffer),
//             ),
//             RELATIVE_ASSIGNABLE_CONTROLLER_CODE => RelativeAssignableController(
//                 RelativeAssignableControllerBorrowed::from_data_unchecked(buffer),
//             ),
//             RELATIVE_REGISTERED_CONTROLLER_CODE => RelativeRegisteredController(
//                 RelativeRegisteredControllerBorrowed::from_data_unchecked(buffer),
//             ),
//             _ => panic!(),
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn channel() {
//         assert_eq!(
//             Midi2ChannelVoiceMessage::from_data(&[0x4BAC_5900, 0xC0B83064, 0x0, 0x0])
//                 .unwrap()
//                 .channel(),
//             u4::new(0xC),
//         );
//     }
// }
