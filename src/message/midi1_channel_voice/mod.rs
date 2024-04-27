pub mod channel_pressure;
pub mod control_change;
// pub mod key_pressure;
// pub mod note_off;
// pub mod note_on;
// pub mod pitch_bend;
// pub mod program_change;

pub const UMP_MESSAGE_TYPE: u8 = 0x2;

// #[derive(
//     derive_more::From,
//     midi2_proc::Data,
//     midi2_proc::Channeled,
//     midi2_proc::Grouped,
//     midi2_proc::UmpDebug,
//     midi2_proc::WriteByteData,
//     Clone,
//     PartialEq,
//     Eq,
// )]
// pub enum Midi1ChannelVoiceMessage<'a> {
//     ChannelPressure(ChannelPressureMessage<'a>),
//     ControlChange(ControlChangeMessage<'a>),
//     KeyPressure(KeyPressureMessage<'a>),
//     NoteOff(NoteOffMessage<'a>),
//     NoteOn(NoteOnMessage<'a>),
//     PitchBend(PitchBendMessage<'a>),
//     ProgramChange(ProgramChangeMessage<'a>),
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn channel() {
//         assert_eq!(
//             Midi1ChannelVoiceMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
//                 .unwrap()
//                 .channel(),
//             u4::new(0x6),
//         );
//     }
// }
