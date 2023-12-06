use crate::{util::BitOps, *};

mod helpers;

pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod sysex_bytes;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod ump_stream;
pub mod utility;

use midi1_channel_voice::Midi1ChannelVoiceBorrowed;
use midi1_channel_voice::Midi1ChannelVoiceBuilder;
use midi1_channel_voice::Midi1ChannelVoiceMessage;
use midi1_channel_voice::Midi1ChannelVoiceOwned;
use midi2_channel_voice::Midi2ChannelVoiceBorrowed;
use midi2_channel_voice::Midi2ChannelVoiceBuilder;
use midi2_channel_voice::Midi2ChannelVoiceMessage;
use midi2_channel_voice::Midi2ChannelVoiceOwned;
use system_common::SystemCommonBorrowed;
use system_common::SystemCommonBuilder;
use system_common::SystemCommonMessage;
use system_common::SystemCommonOwned;
use system_exclusive_7bit::Sysex7Borrowed;
use system_exclusive_7bit::Sysex7Message;
#[cfg(feature = "std")]
use system_exclusive_7bit::Sysex7Owned;
use system_exclusive_8bit::Sysex8Borrowed;
use system_exclusive_8bit::Sysex8Message;
#[cfg(feature = "std")]
use system_exclusive_8bit::Sysex8Owned;
use utility::UtilityBorrowed;
use utility::UtilityBuilder;
use utility::UtilityMessage;
use utility::UtilityOwned;

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Message<'a> {
    Midi1ChannelVoice(Midi1ChannelVoiceMessage<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceMessage<'a>),
    Utility(UtilityMessage<'a>),
    SystemCommon(SystemCommonMessage<'a>),
    Sysex7(Sysex7Message<'a>),
    Sysex8(Sysex8Message<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageBorrowed<'a> {
    Midi1ChannelVoice(Midi1ChannelVoiceBorrowed<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceBorrowed<'a>),
    Utility(UtilityBorrowed<'a>),
    SystemCommon(SystemCommonBorrowed<'a>),
    Sysex7(Sysex7Borrowed<'a>),
    Sysex8(Sysex8Borrowed<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageOwned {
    Midi1ChannelVoice(Midi1ChannelVoiceOwned),
    Midi2ChannelVoice(Midi2ChannelVoiceOwned),
    Utility(UtilityOwned),
    SystemCommon(SystemCommonOwned),
    #[cfg(feature = "std")]
    Sysex7(Sysex7Owned),
    #[cfg(feature = "std")]
    Sysex8(Sysex8Owned),
}

impl<'a> Message<'a> {
    pub fn builder() -> MessageBuilder<Self> {
        MessageBuilder::new()
    }
}

#[derive(Default)]
pub struct MessageBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<utility::time_stamp::TimeStampOwned>
        + core::convert::From<utility::no_op::NoOpOwned>
        + core::convert::From<system_common::active_sensing::ActiveSensingOwned>
        + core::convert::From<system_common::cont::ContinueOwned>
        + core::convert::From<system_common::reset::ResetOwned>
        + core::convert::From<system_common::song_position_pointer::SongPositionPointerOwned>
        + core::convert::From<system_common::song_select::SongSelectOwned>
        + core::convert::From<system_common::start::StartOwned>
        + core::convert::From<system_common::stop::StopOwned>
        + core::convert::From<system_common::time_code::TimeCodeOwned>
        + core::convert::From<system_common::timing_clock::TimingClockOwned>
        + core::convert::From<system_common::tune_request::TuneRequestOwned>
        + core::convert::From<midi2_channel_voice::assignable_controller::AssignableControllerOwned>
        + core::convert::From<
            midi2_channel_voice::assignable_per_note_controller::AssignablePerNoteControllerOwned,
        > + core::convert::From<midi2_channel_voice::channel_pitch_bend::ChannelPitchBendOwned>
        + core::convert::From<midi2_channel_voice::channel_pressure::ChannelPressureOwned>
        + core::convert::From<midi2_channel_voice::control_change::ControlChangeOwned>
        + core::convert::From<midi2_channel_voice::key_pressure::KeyPressureOwned>
        + core::convert::From<midi2_channel_voice::note_off::NoteOffOwned>
        + core::convert::From<midi2_channel_voice::note_on::NoteOnOwned>
        + core::convert::From<midi2_channel_voice::per_note_management::PerNoteManagementOwned>
        + core::convert::From<midi2_channel_voice::per_note_pitch_bend::PerNotePitchBendOwned>
        + core::convert::From<midi2_channel_voice::program_change::ProgramChangeOwned>
        + core::convert::From<midi2_channel_voice::registered_controller::RegisteredControllerOwned>
        + core::convert::From<
            midi2_channel_voice::registered_per_note_controller::RegisteredPerNoteControllerOwned,
        > + core::convert::From<
            midi2_channel_voice::relative_assignable_controller::RelativeAssignableControllerOwned,
        > + core::convert::From<
            midi2_channel_voice::relative_registered_controller::RelativeRegisteredControllerOwned,
        > + core::convert::From<midi1_channel_voice::channel_pressure::ChannelPressureOwned>
        + core::convert::From<midi1_channel_voice::control_change::ControlChangeOwned>
        + core::convert::From<midi1_channel_voice::key_pressure::KeyPressureOwned>
        + core::convert::From<midi1_channel_voice::note_off::NoteOffOwned>
        + core::convert::From<midi1_channel_voice::note_on::NoteOnOwned>
        + core::convert::From<midi1_channel_voice::pitch_bend::PitchBendOwned>
        + core::convert::From<midi1_channel_voice::program_change::ProgramChangeOwned>;

impl<M> MessageBuilder<M>
where
    M: core::convert::From<utility::time_stamp::TimeStampOwned>
        + core::convert::From<utility::no_op::NoOpOwned>
        + core::convert::From<system_common::active_sensing::ActiveSensingOwned>
        + core::convert::From<system_common::cont::ContinueOwned>
        + core::convert::From<system_common::reset::ResetOwned>
        + core::convert::From<system_common::song_position_pointer::SongPositionPointerOwned>
        + core::convert::From<system_common::song_select::SongSelectOwned>
        + core::convert::From<system_common::start::StartOwned>
        + core::convert::From<system_common::stop::StopOwned>
        + core::convert::From<system_common::time_code::TimeCodeOwned>
        + core::convert::From<system_common::timing_clock::TimingClockOwned>
        + core::convert::From<system_common::tune_request::TuneRequestOwned>
        + core::convert::From<midi2_channel_voice::assignable_controller::AssignableControllerOwned>
        + core::convert::From<
            midi2_channel_voice::assignable_per_note_controller::AssignablePerNoteControllerOwned,
        > + core::convert::From<midi2_channel_voice::channel_pitch_bend::ChannelPitchBendOwned>
        + core::convert::From<midi2_channel_voice::channel_pressure::ChannelPressureOwned>
        + core::convert::From<midi2_channel_voice::control_change::ControlChangeOwned>
        + core::convert::From<midi2_channel_voice::key_pressure::KeyPressureOwned>
        + core::convert::From<midi2_channel_voice::note_off::NoteOffOwned>
        + core::convert::From<midi2_channel_voice::note_on::NoteOnOwned>
        + core::convert::From<midi2_channel_voice::per_note_management::PerNoteManagementOwned>
        + core::convert::From<midi2_channel_voice::per_note_pitch_bend::PerNotePitchBendOwned>
        + core::convert::From<midi2_channel_voice::program_change::ProgramChangeOwned>
        + core::convert::From<midi2_channel_voice::registered_controller::RegisteredControllerOwned>
        + core::convert::From<
            midi2_channel_voice::registered_per_note_controller::RegisteredPerNoteControllerOwned,
        > + core::convert::From<
            midi2_channel_voice::relative_assignable_controller::RelativeAssignableControllerOwned,
        > + core::convert::From<
            midi2_channel_voice::relative_registered_controller::RelativeRegisteredControllerOwned,
        > + core::convert::From<midi1_channel_voice::channel_pressure::ChannelPressureOwned>
        + core::convert::From<midi1_channel_voice::control_change::ControlChangeOwned>
        + core::convert::From<midi1_channel_voice::key_pressure::KeyPressureOwned>
        + core::convert::From<midi1_channel_voice::note_off::NoteOffOwned>
        + core::convert::From<midi1_channel_voice::note_on::NoteOnOwned>
        + core::convert::From<midi1_channel_voice::pitch_bend::PitchBendOwned>
        + core::convert::From<midi1_channel_voice::program_change::ProgramChangeOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn midi1_channel_voice(self) -> Midi1ChannelVoiceBuilder<M> {
        Midi1ChannelVoiceBuilder::new()
    }
    pub fn midi2_channel_voice(self) -> Midi2ChannelVoiceBuilder<M> {
        Midi2ChannelVoiceBuilder::new()
    }
    pub fn utility(self) -> UtilityBuilder<M> {
        UtilityBuilder::new()
    }
    pub fn system_common(self) -> SystemCommonBuilder<M> {
        SystemCommonBuilder::new()
    }
}

impl MessageOwned {
    pub fn builder() -> MessageBuilder<Self> {
        MessageBuilder::new()
    }
}

const MIDI1_CHANNEL_VOICE_CODE: u8 = 2;
const MIDI2_CHANNEL_VOICE_CODE: u8 = 4;
const SYSEX7_CODE: u8 = 3;
const SYSEX8_CODE: u8 = 5;
const UTILITY_CODE: u8 = 0;
const SYSTEM_COMMON_CODE: u8 = 1;

impl<'a> FromData<'a> for MessageBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        use MessageBorrowed::*;
        match u8::from(buffer[0].nibble(0)) {
            MIDI1_CHANNEL_VOICE_CODE => {
                Midi1ChannelVoice(Midi1ChannelVoiceBorrowed::from_data_unchecked(buffer))
            }
            MIDI2_CHANNEL_VOICE_CODE => {
                Midi2ChannelVoice(Midi2ChannelVoiceBorrowed::from_data_unchecked(buffer))
            }
            UTILITY_CODE => Utility(UtilityBorrowed::from_data_unchecked(buffer)),
            SYSTEM_COMMON_CODE => SystemCommon(SystemCommonBorrowed::from_data_unchecked(buffer)),
            SYSEX7_CODE => Sysex7(Sysex7Borrowed::from_data_unchecked(buffer)),
            SYSEX8_CODE => Sysex8(Sysex8Borrowed::from_data_unchecked(buffer)),
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(0)) {
            MIDI1_CHANNEL_VOICE_CODE => Midi1ChannelVoiceBorrowed::validate_data(buffer),
            MIDI2_CHANNEL_VOICE_CODE => Midi2ChannelVoiceBorrowed::validate_data(buffer),
            UTILITY_CODE => UtilityBorrowed::validate_data(buffer),
            SYSTEM_COMMON_CODE => SystemCommonBorrowed::validate_data(buffer),
            SYSEX7_CODE => Sysex7Borrowed::validate_data(buffer),
            SYSEX8_CODE => Sysex8Borrowed::validate_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'a> FromData<'a> for Message<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        MessageBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        MessageBorrowed::from_data_unchecked(buffer).into()
    }
}

impl<'a> FromByteData<'a> for MessageOwned {
    type Target = Self;
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        use MessageOwned::*;
        match buffer[0] {
            0x80..=0xEF => {
                Midi1ChannelVoice(Midi1ChannelVoiceOwned::from_byte_data_unchecked(buffer))
            }
            0xF1..=0xF6 => SystemCommon(SystemCommonOwned::from_byte_data_unchecked(buffer)),
            0xF8..=0xFF => SystemCommon(SystemCommonOwned::from_byte_data_unchecked(buffer)),
            0xF0 => todo!(), // sysex begin
            _ => panic!(),
        }
    }
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        match buffer[0] {
            0x80..=0xEF => Midi1ChannelVoiceOwned::validate_byte_data(buffer),
            0xF1..=0xF6 => SystemCommonOwned::validate_byte_data(buffer),
            0xF8..=0xFF => SystemCommonOwned::validate_byte_data(buffer),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'a, 'b> FromByteData<'a> for Message<'b> {
    type Target = Self;
    fn from_byte_data_unchecked(buffer: &'a [u8]) -> Self::Target {
        MessageOwned::from_byte_data_unchecked(buffer).into()
    }
    fn validate_byte_data(buffer: &'a [u8]) -> Result<()> {
        MessageOwned::validate_byte_data(buffer)
    }
}

impl TryWriteByteData for MessageOwned {
    fn try_write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
        use MessageOwned::*;
        match self {
            Midi1ChannelVoice(m) => Ok(m.write_byte_data(buffer)),
            SystemCommon(m) => Ok(m.write_byte_data(buffer)),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'b> TryWriteByteData for MessageBorrowed<'b> {
    fn try_write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
        use MessageBorrowed::*;
        match self {
            Midi1ChannelVoice(m) => Ok(m.write_byte_data(buffer)),
            SystemCommon(m) => Ok(m.write_byte_data(buffer)),
            _ => Err(Error::InvalidData),
        }
    }
}

impl<'b> TryWriteByteData for Message<'b> {
    fn try_write_byte_data<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8]> {
        use Message::*;
        match self {
            Midi1ChannelVoice(m) => Ok(m.write_byte_data(buffer)),
            SystemCommon(m) => Ok(m.write_byte_data(buffer)),
            _ => Err(Error::InvalidData),
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> TryIntoOwned for MessageBorrowed<'a> {
    type Owned = MessageOwned;
    fn try_into_owned(self) -> Result<Self::Owned> {
        use MessageBorrowed as B;
        use MessageOwned as O;
        match self {
            B::Midi1ChannelVoice(m) => Ok(O::Midi1ChannelVoice(m.into_owned())),
            B::Midi2ChannelVoice(m) => Ok(O::Midi2ChannelVoice(m.into_owned())),
            B::Utility(m) => Ok(O::Utility(m.into_owned())),
            B::SystemCommon(m) => Ok(O::SystemCommon(m.into_owned())),
            _ => Err(Error::InvalidData),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for MessageBorrowed<'a> {
    type Owned = MessageOwned;
    fn into_owned(self) -> Self::Owned {
        use MessageBorrowed as B;
        use MessageOwned as O;
        match self {
            B::Midi1ChannelVoice(m) => O::Midi1ChannelVoice(m.into_owned()),
            B::Midi2ChannelVoice(m) => O::Midi2ChannelVoice(m.into_owned()),
            B::Utility(m) => O::Utility(m.into_owned()),
            B::SystemCommon(m) => O::SystemCommon(m.into_owned()),
            B::Sysex7(m) => O::Sysex7(m.into_owned()),
            B::Sysex8(m) => O::Sysex8(m.into_owned()),
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> TryIntoOwned for Message<'a> {
    type Owned = MessageOwned;
    fn try_into_owned(self) -> Result<Self::Owned> {
        use Message as M;
        use MessageOwned as O;
        match self {
            M::Midi1ChannelVoice(m) => Ok(O::Midi1ChannelVoice(m.into_owned())),
            M::Midi2ChannelVoice(m) => Ok(O::Midi2ChannelVoice(m.into_owned())),
            M::Utility(m) => Ok(O::Utility(m.into_owned())),
            M::SystemCommon(m) => Ok(O::SystemCommon(m.into_owned())),
            _ => Err(Error::InvalidData),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for Message<'a> {
    type Owned = MessageOwned;
    fn into_owned(self) -> Self::Owned {
        use Message as M;
        use MessageOwned as O;
        match self {
            M::Midi1ChannelVoice(m) => O::Midi1ChannelVoice(m.into_owned()),
            M::Midi2ChannelVoice(m) => O::Midi2ChannelVoice(m.into_owned()),
            M::Utility(m) => O::Utility(m.into_owned()),
            M::SystemCommon(m) => O::SystemCommon(m.into_owned()),
            M::Sysex7(m) => O::Sysex7(m.into_owned()),
            M::Sysex8(m) => O::Sysex8(m.into_owned()),
        }
    }
}

impl<'a> core::convert::From<MessageBorrowed<'a>> for Message<'a> {
    fn from(value: MessageBorrowed<'a>) -> Self {
        use Message as M;
        use MessageBorrowed as B;
        match value {
            B::Midi1ChannelVoice(m) => M::Midi1ChannelVoice(m.into()),
            B::Midi2ChannelVoice(m) => M::Midi2ChannelVoice(m.into()),
            B::Utility(m) => M::Utility(m.into()),
            B::SystemCommon(m) => M::SystemCommon(m.into()),
            B::Sysex7(m) => M::Sysex7(m.into()),
            B::Sysex8(m) => M::Sysex8(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> core::convert::From<MessageOwned> for Message<'a> {
    fn from(value: MessageOwned) -> Self {
        use Message as M;
        use MessageOwned as O;
        match value {
            O::Midi1ChannelVoice(m) => M::Midi1ChannelVoice(m.into()),
            O::Midi2ChannelVoice(m) => M::Midi2ChannelVoice(m.into()),
            O::Utility(m) => M::Utility(m.into()),
            O::SystemCommon(m) => M::SystemCommon(m.into()),
            O::Sysex7(m) => M::Sysex7(m.into()),
            O::Sysex8(m) => M::Sysex8(m.into()),
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> core::convert::From<MessageOwned> for Message<'a> {
    fn from(value: MessageOwned) -> Self {
        use Message as M;
        use MessageOwned as O;
        match value {
            O::Midi1ChannelVoice(m) => M::Midi1ChannelVoice(m.into()),
            O::Midi2ChannelVoice(m) => M::Midi2ChannelVoice(m.into()),
            O::Utility(m) => M::Utility(m.into()),
            O::SystemCommon(m) => M::SystemCommon(m.into()),
        }
    }
}

macro_rules! from_message_impl {
    ($message: ty, $intermediate_message: ty) => {
        impl core::convert::From<$message> for MessageOwned {
            fn from(value: $message) -> Self {
                <$intermediate_message as core::convert::From<$message>>::from(value).into()
            }
        }
        impl<'a> core::convert::From<$message> for Message<'a> {
            fn from(value: $message) -> Self {
                <MessageOwned as core::convert::From<$message>>::from(value).into()
            }
        }
    };
}

macro_rules! from_utility_message_impl {
    ($message: ty) => {
        from_message_impl!($message, UtilityOwned);
    };
}

from_utility_message_impl!(utility::no_op::NoOpOwned);
from_utility_message_impl!(utility::time_stamp::TimeStampOwned);

macro_rules! from_system_common_message_impl {
    ($message: ty) => {
        from_message_impl!($message, SystemCommonOwned);
    };
}

from_system_common_message_impl!(system_common::active_sensing::ActiveSensingOwned);
from_system_common_message_impl!(system_common::cont::ContinueOwned);
from_system_common_message_impl!(system_common::reset::ResetOwned);
from_system_common_message_impl!(system_common::song_position_pointer::SongPositionPointerOwned);
from_system_common_message_impl!(system_common::song_select::SongSelectOwned);
from_system_common_message_impl!(system_common::start::StartOwned);
from_system_common_message_impl!(system_common::stop::StopOwned);
from_system_common_message_impl!(system_common::time_code::TimeCodeOwned);
from_system_common_message_impl!(system_common::timing_clock::TimingClockOwned);
from_system_common_message_impl!(system_common::tune_request::TuneRequestOwned);

macro_rules! from_midi1_channel_voice_message_impl {
    ($message: ty) => {
        from_message_impl!($message, Midi1ChannelVoiceOwned);
    };
}

from_midi1_channel_voice_message_impl!(midi1_channel_voice::channel_pressure::ChannelPressureOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::control_change::ControlChangeOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::key_pressure::KeyPressureOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::note_off::NoteOffOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::note_on::NoteOnOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::pitch_bend::PitchBendOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::program_change::ProgramChangeOwned);

macro_rules! from_midi2_channel_voice_message_impl {
    ($message: ty) => {
        from_message_impl!($message, Midi2ChannelVoiceOwned);
    };
}

from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::assignable_controller::AssignableControllerOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::assignable_per_note_controller::AssignablePerNoteControllerOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::channel_pitch_bend::ChannelPitchBendOwned
);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::channel_pressure::ChannelPressureOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::control_change::ControlChangeOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::key_pressure::KeyPressureOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::note_off::NoteOffOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::note_on::NoteOnOwned);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::per_note_management::PerNoteManagementOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::per_note_pitch_bend::PerNotePitchBendOwned
);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::program_change::ProgramChangeOwned);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::registered_controller::RegisteredControllerOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::registered_per_note_controller::RegisteredPerNoteControllerOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::relative_assignable_controller::RelativeAssignableControllerOwned
);
from_midi2_channel_voice_message_impl!(
    midi2_channel_voice::relative_registered_controller::RelativeRegisteredControllerOwned
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_byte_data() {
        assert_eq!(
            Message::from_byte_data(&[0xAB, 0x60, 0x33]),
            Message::builder()
                .midi1_channel_voice()
                .key_pressure()
                .channel(u4::new(0xB))
                .note(u7::new(0x60))
                .pressure(u7::new(0x33))
                .build(),
        );
    }
}
