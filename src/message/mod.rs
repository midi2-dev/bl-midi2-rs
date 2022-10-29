use crate::packet::{Packet, PacketMethods};

mod helpers;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod utility;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Message {
    Midi1ChannelPressure(midi1_channel_voice::ChannelPressureMessage),
    Midi1ControlChange(midi1_channel_voice::ControlChangeMessage),
    Midi1KeyPressure(midi1_channel_voice::KeyPressureMessage),
    Midi1NoteOff(midi1_channel_voice::NoteOffMessage),
    Midi1NoteOn(midi1_channel_voice::NoteOnMessage),
    Midi1PitchBend(midi1_channel_voice::PitchBendMessage),
    Midi1ProgramChange(midi1_channel_voice::ProgramChangeMessage),
    Midi2PitchBend(midi2_channel_voice::PitchBendMessage),
    Midi2ChannelPressure(midi2_channel_voice::ChannelPressureMessage),
    Midi2AssignableController(midi2_channel_voice::AssignableControllerMessage),
    Midi2RegisteredController(midi2_channel_voice::RegisteredControllerMessage),
    Midi2RelativeAssignableController(midi2_channel_voice::RelativeAssignableControllerMessage),
    Midi2RelativeRegisteredController(midi2_channel_voice::RelativeRegisteredControllerMessage),
    Midi2NoteOff(midi2_channel_voice::NoteOffMessage),
    Midi2NoteOn(midi2_channel_voice::NoteOnMessage),
    Midi2PerNotePitchBend(midi2_channel_voice::PerNotePitchBendMessage),
    Midi2AssignablePerNoteController(midi2_channel_voice::AssignablePerNoteControllerMessage),
    Midi2ControlChange(midi2_channel_voice::ControlChangeMessage),
    Midi2PerNoteManagement(midi2_channel_voice::PerNoteManagementMessage),
    Midi2ProgramChange(midi2_channel_voice::ProgramChangeMessage),
    Midi2RegisteredPerNoteController(midi2_channel_voice::RegisteredPerNoteControllerMessage),
    Midi2KeyPressure(midi2_channel_voice::KeyPressureMessage),
    SystemCommonActiveSensing(system_common::ActiveSensingMessage),
    SystemCommonContinue(system_common::ContinueMessage),
    SystemCommonReset(system_common::ResetMessage),
    SystemCommonSongPositionPointer(system_common::SongPositionPointerMessage),
    SystemCommonSongSelect(system_common::SongSelectMessage),
    SystemCommonStart(system_common::StartMessage),
    SystemCommonStop(system_common::StopMessage),
    SystemCommonTimeCode(system_common::TimeCodeMessage),
    SystemCommonTimingClock(system_common::TimingClockMessage),
    SystemCommonTuneRequest(system_common::TuneRequestMessage),
    UtilityNoOp(utility::NoOpMessage),
    UtilityTimeStamp(utility::TimeStampMessage),
    SysEx7(system_exclusive_7bit::Message),
    SysEx8(system_exclusive_8bit::Message),
}

fn write_type_to_packet(t: ux::u4, p: &mut Packet) {
    p.set_nibble(0, t);
}

fn write_group_to_packet(g: ux::u4, p: &mut Packet) {
    p.set_nibble(1, g);
}