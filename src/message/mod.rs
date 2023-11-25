use crate::{util::BitOps, *};

mod helpers;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod utility;

pub use midi1_channel_voice::Midi1ChannelVoiceBorrowed;
pub use midi1_channel_voice::Midi1ChannelVoiceBuilder;
pub use midi1_channel_voice::Midi1ChannelVoiceOwned;
pub use midi2_channel_voice::Midi2ChannelVoiceBorrowed;
pub use midi2_channel_voice::Midi2ChannelVoiceBuilder;
pub use midi2_channel_voice::Midi2ChannelVoiceOwned;
pub use system_common::SystemCommonBorrowed;
pub use system_common::SystemCommonBuilder;
pub use system_common::SystemCommonOwned;
pub use system_exclusive_7bit::Sysex7Borrowed;
pub use system_exclusive_7bit::Sysex7Owned;
pub use system_exclusive_8bit::Sysex8Borrowed;
pub use system_exclusive_8bit::Sysex8Owned;
pub use utility::UtilityBorrowed;
pub use utility::UtilityOwned;

use self::utility::UtilityBuilder;

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
pub enum MessageBorrowed<'a> {
    Midi1ChannelVoice(Midi1ChannelVoiceBorrowed<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceBorrowed<'a>),
    Sysex7(Sysex7Borrowed<'a>),
    Sysex8(Sysex8Borrowed<'a>),
    Utility(UtilityBorrowed<'a>),
    SystemCommon(SystemCommonBorrowed<'a>),
}

#[derive(derive_more::From, Clone, Debug, PartialEq, Eq)]
pub enum MessageOwned {
    Midi1ChannelVoice(Midi1ChannelVoiceOwned),
    Midi2ChannelVoice(Midi2ChannelVoiceOwned),
    Sysex7(Sysex7Owned),
    Sysex8(Sysex8Owned),
    Utility(UtilityOwned),
    SystemCommon(SystemCommonOwned),
}

#[derive(Default)]
pub struct MessageBuilder;

impl MessageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn midi1_channel_voice(self) -> Midi1ChannelVoiceBuilder<MessageOwned> {
        Midi1ChannelVoiceBuilder::new()
    }
    pub fn midi2_channel_voice(self) -> Midi2ChannelVoiceBuilder<MessageOwned> {
        Midi2ChannelVoiceBuilder::new()
    }
    pub fn utility(self) -> UtilityBuilder<MessageOwned> {
        UtilityBuilder::new()
    }
    pub fn system_common(self) -> SystemCommonBuilder<MessageOwned> {
        SystemCommonBuilder::new()
    }
}

impl MessageOwned {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

const MIDI1_CHANNEL_VOICE_CODE: u8 = 2;
const MIDI2_CHANNEL_VOICE_CODE: u8 = 4;
const SYSEX7_CODE: u8 = 3;
const SYSEX8_CODE: u8 = 5;
const UTILITY_CODE: u8 = 0;
const SYSTEM_COMMON_CODE: u8 = 1;

impl<'a> Data for MessageBorrowed<'a> {
    fn data(&self) -> &[u32] {
        use MessageBorrowed::*;
        match self {
            Midi1ChannelVoice(m) => m.data(),
            Midi2ChannelVoice(m) => m.data(),
            Sysex7(m) => m.data(),
            Sysex8(m) => m.data(),
            Utility(m) => m.data(),
            SystemCommon(m) => m.data(),
        }
    }
}

impl Data for MessageOwned {
    fn data(&self) -> &[u32] {
        use MessageOwned::*;
        match self {
            Midi1ChannelVoice(m) => m.data(),
            Midi2ChannelVoice(m) => m.data(),
            Sysex7(m) => m.data(),
            Sysex8(m) => m.data(),
            Utility(m) => m.data(),
            SystemCommon(m) => m.data(),
        }
    }
}

impl<'a> Grouped for MessageBorrowed<'a> {
    fn group(&self) -> u4 {
        use MessageBorrowed::*;
        match self {
            Midi1ChannelVoice(m) => m.group(),
            Midi2ChannelVoice(m) => m.group(),
            Sysex7(m) => m.group(),
            Sysex8(m) => m.group(),
            Utility(m) => m.group(),
            SystemCommon(m) => m.group(),
        }
    }
}

impl Grouped for MessageOwned {
    fn group(&self) -> u4 {
        use MessageOwned::*;
        match self {
            Midi1ChannelVoice(m) => m.group(),
            Midi2ChannelVoice(m) => m.group(),
            Sysex7(m) => m.group(),
            Sysex8(m) => m.group(),
            Utility(m) => m.group(),
            SystemCommon(m) => m.group(),
        }
    }
}

impl<'a> FromData<'a> for MessageBorrowed<'a> {
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        use MessageBorrowed::*;
        match u8::from(buffer[0].nibble(0)) {
            MIDI1_CHANNEL_VOICE_CODE => {
                Midi1ChannelVoice(Midi1ChannelVoiceBorrowed::from_data_unchecked(buffer))
            }
            MIDI2_CHANNEL_VOICE_CODE => {
                Midi2ChannelVoice(Midi2ChannelVoiceBorrowed::from_data_unchecked(buffer))
            }
            SYSEX7_CODE => Sysex7(Sysex7Borrowed::from_data_unchecked(buffer)),
            SYSEX8_CODE => Sysex8(Sysex8Borrowed::from_data_unchecked(buffer)),
            UTILITY_CODE => Utility(UtilityBorrowed::from_data_unchecked(buffer)),
            SYSTEM_COMMON_CODE => SystemCommon(SystemCommonBorrowed::from_data_unchecked(buffer)),
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(0)) {
            MIDI1_CHANNEL_VOICE_CODE => Midi1ChannelVoiceBorrowed::validate_data(buffer),
            MIDI2_CHANNEL_VOICE_CODE => Midi2ChannelVoiceBorrowed::validate_data(buffer),
            SYSEX7_CODE => Sysex7Borrowed::validate_data(buffer),
            SYSEX8_CODE => Sysex8Borrowed::validate_data(buffer),
            UTILITY_CODE => UtilityBorrowed::validate_data(buffer),
            SYSTEM_COMMON_CODE => SystemCommonBorrowed::validate_data(buffer),
            _ => Err(Error::InvalidData),
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
    };
}

macro_rules! from_utility_message_impl {
    ($message: ty) => {
        from_message_impl!($message, UtilityOwned);
    };
}

from_utility_message_impl!(utility::NoOpOwned);
from_utility_message_impl!(utility::TimeStampOwned);

macro_rules! from_system_common_message_impl {
    ($message: ty) => {
        from_message_impl!($message, SystemCommonOwned);
    };
}

from_system_common_message_impl!(system_common::ActiveSensingOwned);
from_system_common_message_impl!(system_common::ContinueOwned);
from_system_common_message_impl!(system_common::ResetOwned);
from_system_common_message_impl!(system_common::SongPositionPointerOwned);
from_system_common_message_impl!(system_common::SongSelectOwned);
from_system_common_message_impl!(system_common::StartOwned);
from_system_common_message_impl!(system_common::StopOwned);
from_system_common_message_impl!(system_common::TimeCodeOwned);
from_system_common_message_impl!(system_common::TimingClockOwned);
from_system_common_message_impl!(system_common::TuneRequestOwned);

macro_rules! from_midi1_channel_voice_message_impl {
    ($message: ty) => {
        from_message_impl!($message, Midi1ChannelVoiceOwned);
    };
}

from_midi1_channel_voice_message_impl!(midi1_channel_voice::ChannelPressureOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::ControlChangeOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::KeyPressureOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::NoteOffOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::NoteOnOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::PitchBendOwned);
from_midi1_channel_voice_message_impl!(midi1_channel_voice::ProgramChangeOwned);

macro_rules! from_midi2_channel_voice_message_impl {
    ($message: ty) => {
        from_message_impl!($message, Midi2ChannelVoiceOwned);
    };
}

from_midi2_channel_voice_message_impl!(midi2_channel_voice::AssignableControllerOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::AssignablePerNoteControllerOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::ChannelPitchBendOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::ChannelPressureOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::ControlChangeOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::KeyPressureOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::NoteOffOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::NoteOnOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::PerNoteManagementOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::PerNotePitchBendOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::ProgramChangeOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::RegisteredControllerOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::RegisteredPerNoteControllerOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::RelativeAssignableControllerOwned);
from_midi2_channel_voice_message_impl!(midi2_channel_voice::RelativeRegisteredControllerOwned);
