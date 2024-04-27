pub mod channel_pressure;
pub mod control_change;
pub mod key_pressure;
pub mod note_off;
pub mod note_on;
pub mod pitch_bend;
pub mod program_change;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x2;

#[derive(derive_more::From, midi2_proc::Data, midi2_proc::Channeled, Debug, PartialEq, Eq)]
pub enum Midi1ChannelVoice<B: crate::buffer::Buffer> {
    ChannelPressure(channel_pressure::ChannelPressure<B>),
    ControlChange(control_change::ControlChange<B>),
    KeyPressure(key_pressure::KeyPressure<B>),
    NoteOff(note_off::NoteOff<B>),
    NoteOn(note_on::NoteOn<B>),
    PitchBend(pitch_bend::PitchBend<B>),
    ProgramChange(program_change::ProgramChange<B>),
}

impl<B: crate::buffer::Ump> crate::traits::Grouped<B> for Midi1ChannelVoice<B> {
    fn group(&self) -> crate::u4 {
        use Midi1ChannelVoice::*;
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
    fn set_group(&mut self, group: crate::u4)
    where
        B: crate::buffer::BufferMut,
    {
        use Midi1ChannelVoice::*;
        match self {
            ChannelPressure(m) => m.set_group(group),
            ControlChange(m) => m.set_group(group),
            KeyPressure(m) => m.set_group(group),
            NoteOff(m) => m.set_group(group),
            NoteOn(m) => m.set_group(group),
            PitchBend(m) => m.set_group(group),
            ProgramChange(m) => m.set_group(group),
        }
    }
}

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
