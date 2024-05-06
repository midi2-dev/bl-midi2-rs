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

pub use assignable_controller::AssignableController;
pub use assignable_per_note_controller::AssignablePerNoteController;
pub use attribute::Attribute as NoteAttribute;
pub use channel_pitch_bend::ChannelPitchBend;
pub use channel_pressure::ChannelPressure;
pub use control_change::ControlChange;
pub use controller::Controller;
pub use key_pressure::KeyPressure;
pub use note_off::NoteOff;
pub use note_on::NoteOn;
pub use per_note_management::PerNoteManagement;
pub use per_note_pitch_bend::PerNotePitchBend;
pub use program_change::ProgramChange;
pub use registered_controller::RegisteredController;
pub use registered_per_note_controller::RegisteredPerNoteController;
pub use relative_assignable_controller::RelativeAssignableController;
pub use relative_registered_controller::RelativeRegisteredController;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x4;

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Channeled,
    midi2_proc::Grouped,
    midi2_proc::JitterReduced,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum ChannelVoice2<B: crate::buffer::Ump> {
    AssignableController(assignable_controller::AssignableController<B>),
    AssignablePerNoteController(assignable_per_note_controller::AssignablePerNoteController<B>),
    ChannelPitchBend(channel_pitch_bend::ChannelPitchBend<B>),
    ChannelPressure(channel_pressure::ChannelPressure<B>),
    ControlChange(control_change::ControlChange<B>),
    KeyPressure(key_pressure::KeyPressure<B>),
    NoteOff(note_off::NoteOff<B>),
    NoteOn(note_on::NoteOn<B>),
    PerNoteManagement(per_note_management::PerNoteManagement<B>),
    PerNotePitchBend(per_note_pitch_bend::PerNotePitchBend<B>),
    ProgramChange(program_change::ProgramChange<B>),
    RegisteredController(registered_controller::RegisteredController<B>),
    RegisteredPerNoteController(registered_per_note_controller::RegisteredPerNoteController<B>),
    RelativeAssignableController(relative_assignable_controller::RelativeAssignableController<B>),
    RelativeRegisteredController(relative_registered_controller::RelativeRegisteredController<B>),
}

impl<'a> TryFrom<&'a [u32]> for ChannelVoice2<&'a [u32]> {
    type Error = crate::error::Error;
    fn try_from(buffer: &'a [u32]) -> Result<Self, Self::Error> {
        if buffer.len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };

        use crate::detail::BitOps;

        Ok(match u8::from(buffer[0].nibble(2)) {
            assignable_controller::STATUS => {
                assignable_controller::AssignableController::try_from(buffer)?.into()
            }
            assignable_per_note_controller::STATUS => {
                assignable_per_note_controller::AssignablePerNoteController::try_from(buffer)?
                    .into()
            }
            channel_pitch_bend::STATUS => {
                channel_pitch_bend::ChannelPitchBend::try_from(buffer)?.into()
            }
            channel_pressure::STATUS => channel_pressure::ChannelPressure::try_from(buffer)?.into(),
            control_change::STATUS => control_change::ControlChange::try_from(buffer)?.into(),
            key_pressure::STATUS => key_pressure::KeyPressure::try_from(buffer)?.into(),
            note_off::STATUS => note_off::NoteOff::try_from(buffer)?.into(),
            note_on::STATUS => note_on::NoteOn::try_from(buffer)?.into(),
            per_note_management::STATUS => {
                per_note_management::PerNoteManagement::try_from(buffer)?.into()
            }
            per_note_pitch_bend::STATUS => {
                per_note_pitch_bend::PerNotePitchBend::try_from(buffer)?.into()
            }
            program_change::STATUS => program_change::ProgramChange::try_from(buffer)?.into(),
            registered_controller::STATUS => {
                registered_controller::RegisteredController::try_from(buffer)?.into()
            }
            registered_per_note_controller::STATUS => {
                registered_per_note_controller::RegisteredPerNoteController::try_from(buffer)?
                    .into()
            }
            relative_assignable_controller::STATUS => {
                relative_assignable_controller::RelativeAssignableController::try_from(buffer)?
                    .into()
            }
            relative_registered_controller::STATUS => {
                relative_registered_controller::RelativeRegisteredController::try_from(buffer)?
                    .into()
            }
            _ => Err(crate::error::Error::InvalidData(
                "Unknown midi2 channel voice status",
            ))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn channel() {
        use crate::traits::Channeled;
        use crate::ux::u4;

        assert_eq!(
            ChannelVoice2::try_from(&[0x4BAC_5900, 0xC0B83064][..])
                .unwrap()
                .channel(),
            u4::new(0xC),
        );
    }
}
