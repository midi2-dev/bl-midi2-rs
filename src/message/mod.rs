use crate::{util::BitOps, *};

mod helpers;

#[cfg(feature = "flex-data")]
pub mod flex_data;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod sysex_bytes;
pub mod system_common;
pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod ump_stream;
pub mod utility;

#[cfg(feature = "flex-data")]
use flex_data::FlexDataBorrowed;
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
use flex_data::FlexDataBuilder;
#[cfg(feature = "flex-data")]
use flex_data::FlexDataMessage;
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
use flex_data::FlexDataOwned;
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
#[cfg(feature = "std")]
use system_exclusive_7bit::Sysex7Builder;
use system_exclusive_7bit::Sysex7Message;
#[cfg(feature = "std")]
use system_exclusive_7bit::Sysex7Owned;
use system_exclusive_8bit::Sysex8Borrowed;
#[cfg(feature = "std")]
use system_exclusive_8bit::Sysex8Builder;
use system_exclusive_8bit::Sysex8Message;
#[cfg(feature = "std")]
use system_exclusive_8bit::Sysex8Owned;
use ump_stream::UmpStreamBorrowed;
#[cfg(feature = "std")]
use ump_stream::UmpStreamBuilder;
use ump_stream::UmpStreamMessage;
#[cfg(feature = "std")]
use ump_stream::UmpStreamOwned;
use utility::UtilityBorrowed;
use utility::UtilityBuilder;
use utility::UtilityMessage;
use utility::UtilityOwned;

#[derive(midi2_attr::UmpDebug, derive_more::From, midi2_attr::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Message<'a> {
    #[cfg(feature = "flex-data")]
    FlexData(FlexDataMessage<'a>),
    Midi1ChannelVoice(Midi1ChannelVoiceMessage<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceMessage<'a>),
    Sysex7(Sysex7Message<'a>),
    Sysex8(Sysex8Message<'a>),
    SystemCommon(SystemCommonMessage<'a>),
    UmpStream(UmpStreamMessage<'a>),
    Utility(UtilityMessage<'a>),
}

#[derive(midi2_attr::UmpDebug, derive_more::From, midi2_attr::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageBorrowed<'a> {
    #[cfg(feature = "flex-data")]
    FlexData(FlexDataBorrowed<'a>),
    Midi1ChannelVoice(Midi1ChannelVoiceBorrowed<'a>),
    Midi2ChannelVoice(Midi2ChannelVoiceBorrowed<'a>),
    Sysex7(Sysex7Borrowed<'a>),
    Sysex8(Sysex8Borrowed<'a>),
    SystemCommon(SystemCommonBorrowed<'a>),
    UmpStream(UmpStreamBorrowed<'a>),
    Utility(UtilityBorrowed<'a>),
}

#[derive(midi2_attr::UmpDebug, derive_more::From, midi2_attr::Data, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageOwned {
    #[cfg(feature = "std")]
    #[cfg(feature = "flex-data")]
    FlexData(FlexDataOwned),
    Midi1ChannelVoice(Midi1ChannelVoiceOwned),
    Midi2ChannelVoice(Midi2ChannelVoiceOwned),
    SystemCommon(SystemCommonOwned),
    #[cfg(feature = "std")]
    Sysex7(Sysex7Owned),
    #[cfg(feature = "std")]
    Sysex8(Sysex8Owned),
    #[cfg(feature = "std")]
    UmpStream(UmpStreamOwned),
    Utility(UtilityOwned),
}

impl<'a> Message<'a> {
    pub fn builder() -> MessageBuilder<Self> {
        MessageBuilder::new()
    }
}

#[derive(Default)]
pub struct MessageBuilder<M>(core::marker::PhantomData<M>);

impl<M> MessageBuilder<M> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[cfg(feature = "std")]
    #[cfg(feature = "flex-data")]
    pub fn flex_data(self) -> FlexDataBuilder<M>
    where
        M: core::convert::From<flex_data::set_chord_name::SetChordNameOwned>
            + core::convert::From<flex_data::set_key_signature::SetKeySignatureOwned>
            + core::convert::From<flex_data::set_metronome::SetMetronomeOwned>
            + core::convert::From<flex_data::set_tempo::SetTempoOwned>
            + core::convert::From<flex_data::set_time_signature::SetTimeSignatureOwned>
            + core::convert::From<flex_data::unknown_metadata_text::UnknownMetadataTextOwned>
            + core::convert::From<flex_data::project_name::ProjectNameOwned>
            + core::convert::From<flex_data::composition_name::CompositionNameOwned>
            + core::convert::From<flex_data::midi_clip_name::MidiClipNameOwned>
            + core::convert::From<flex_data::copyright_notice::CopyrightNoticeOwned>
            + core::convert::From<flex_data::composer_name::ComposerNameOwned>
            + core::convert::From<flex_data::lyricist_name::LyricistNameOwned>
            + core::convert::From<flex_data::arranger_name::ArrangerNameOwned>
            + core::convert::From<flex_data::publisher_name::PublisherNameOwned>
            + core::convert::From<flex_data::primary_performer_name::PrimaryPerformerNameOwned>
            + core::convert::From<
                flex_data::accompanying_performer_name::AccompanyingPerformerNameOwned,
            > + core::convert::From<flex_data::recording_date::RecordingDateOwned>
            + core::convert::From<flex_data::recording_location::RecordingLocationOwned>
            + core::convert::From<flex_data::unknown_performance_text::UnknownPerformanceTextOwned>
            + core::convert::From<flex_data::lyrics::LyricsOwned>
            + core::convert::From<flex_data::lyrics_language::LyricsLanguageOwned>
            + core::convert::From<flex_data::ruby::RubyOwned>
            + core::convert::From<flex_data::ruby_language::RubyLanguageOwned>,
    {
        FlexDataBuilder::new()
    }

    pub fn midi1_channel_voice(self) -> Midi1ChannelVoiceBuilder<M>
    where
        M: core::convert::From<midi1_channel_voice::channel_pressure::ChannelPressureOwned>
            + core::convert::From<midi1_channel_voice::control_change::ControlChangeOwned>
            + core::convert::From<midi1_channel_voice::key_pressure::KeyPressureOwned>
            + core::convert::From<midi1_channel_voice::note_off::NoteOffOwned>
            + core::convert::From<midi1_channel_voice::note_on::NoteOnOwned>
            + core::convert::From<midi1_channel_voice::pitch_bend::PitchBendOwned>
            + core::convert::From<midi1_channel_voice::program_change::ProgramChangeOwned>,
    {
        Midi1ChannelVoiceBuilder::new()
    }
    pub fn midi2_channel_voice(self) -> Midi2ChannelVoiceBuilder<M> where M: core::convert::From<midi2_channel_voice::channel_pitch_bend::ChannelPitchBendOwned>
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
        > + core::convert::From<midi2_channel_voice::assignable_controller::AssignableControllerOwned>
        + core::convert::From<
            midi2_channel_voice::assignable_per_note_controller::AssignablePerNoteControllerOwned,
    >{
        Midi2ChannelVoiceBuilder::new()
    }
    pub fn utility(self) -> UtilityBuilder<M>
    where
        M: core::convert::From<utility::time_stamp::TimeStampOwned>
            + core::convert::From<utility::no_op::NoOpOwned>,
    {
        UtilityBuilder::new()
    }
    pub fn system_common(self) -> SystemCommonBuilder<M>
    where
        M: core::convert::From<system_common::active_sensing::ActiveSensingOwned>
            + core::convert::From<system_common::cont::ContinueOwned>
            + core::convert::From<system_common::reset::ResetOwned>
            + core::convert::From<system_common::song_position_pointer::SongPositionPointerOwned>
            + core::convert::From<system_common::song_select::SongSelectOwned>
            + core::convert::From<system_common::start::StartOwned>
            + core::convert::From<system_common::stop::StopOwned>
            + core::convert::From<system_common::time_code::TimeCodeOwned>
            + core::convert::From<system_common::timing_clock::TimingClockOwned>
            + core::convert::From<system_common::tune_request::TuneRequestOwned>,
    {
        SystemCommonBuilder::new()
    }
    #[cfg(feature = "std")]
    pub fn ump_stream(self) -> UmpStreamBuilder<M>
    where
        M: core::convert::From<ump_stream::device_identity::DeviceIdentityOwned>
            + core::convert::From<ump_stream::end_of_clip::EndOfClipOwned>
            + core::convert::From<ump_stream::endpoint_discovery::EndpointDiscoveryOwned>
            + core::convert::From<ump_stream::endpoint_info::EndpointInfoOwned>
            + core::convert::From<ump_stream::endpoint_name::EndpointNameOwned>
            + core::convert::From<ump_stream::function_block_discovery::FunctionBlockDiscoveryOwned>
            + core::convert::From<ump_stream::function_block_info::FunctionBlockInfoOwned>
            + core::convert::From<ump_stream::function_block_name::FunctionBlockNameOwned>
            + core::convert::From<ump_stream::product_instance_id::ProductInstanceIdOwned>
            + core::convert::From<ump_stream::start_of_clip::StartOfClipOwned>
            + core::convert::From<
                ump_stream::stream_configuration_notification::StreamConfigurationNotificationOwned,
            > + core::convert::From<
                ump_stream::stream_configuration_request::StreamConfigurationRequestOwned,
            >,
    {
        UmpStreamBuilder::new()
    }

    #[cfg(feature = "std")]
    pub fn sysex8(self) -> Sysex8Builder<M>
    where
        M: core::convert::From<Sysex8Owned>,
    {
        Sysex8Builder::new()
    }

    #[cfg(feature = "std")]
    pub fn sysex7(self) -> Sysex7Builder<M>
    where
        M: core::convert::From<Sysex7Owned>,
    {
        Sysex7Builder::new()
    }
}

impl MessageOwned {
    pub fn builder() -> MessageBuilder<Self> {
        MessageBuilder::new()
    }
}

#[cfg(feature = "flex-data")]
const FLEX_DATA_CODE: u8 = 0xD;
const MIDI1_CHANNEL_VOICE_CODE: u8 = 2;
const MIDI2_CHANNEL_VOICE_CODE: u8 = 4;
const SYSEX7_CODE: u8 = 3;
const SYSEX8_CODE: u8 = 5;
const UTILITY_CODE: u8 = 0;
const SYSTEM_COMMON_CODE: u8 = 1;
const UMP_STREAM_CODE: u8 = 0xF;

impl<'a> FromData<'a> for MessageBorrowed<'a> {
    type Target = Self;
    fn from_data_unchecked(buffer: &'a [u32]) -> Self {
        use MessageBorrowed::*;
        match u8::from(buffer[0].nibble(0)) {
            #[cfg(feature = "flex-data")]
            FLEX_DATA_CODE => FlexData(FlexDataBorrowed::from_data_unchecked(buffer)),
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
            UMP_STREAM_CODE => UmpStream(UmpStreamBorrowed::from_data_unchecked(buffer)),
            _ => panic!(),
        }
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        match u8::from(buffer[0].nibble(0)) {
            #[cfg(feature = "flex-data")]
            FLEX_DATA_CODE => FlexDataBorrowed::validate_data(buffer),
            MIDI1_CHANNEL_VOICE_CODE => Midi1ChannelVoiceBorrowed::validate_data(buffer),
            MIDI2_CHANNEL_VOICE_CODE => Midi2ChannelVoiceBorrowed::validate_data(buffer),
            UTILITY_CODE => UtilityBorrowed::validate_data(buffer),
            SYSTEM_COMMON_CODE => SystemCommonBorrowed::validate_data(buffer),
            SYSEX7_CODE => Sysex7Borrowed::validate_data(buffer),
            SYSEX8_CODE => Sysex8Borrowed::validate_data(buffer),
            UMP_STREAM_CODE => UmpStreamBorrowed::validate_data(buffer),
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
            #[cfg(feature = "flex-data")]
            B::FlexData(m) => O::FlexData(m.into_owned()),
            B::Midi1ChannelVoice(m) => O::Midi1ChannelVoice(m.into_owned()),
            B::Midi2ChannelVoice(m) => O::Midi2ChannelVoice(m.into_owned()),
            B::Utility(m) => O::Utility(m.into_owned()),
            B::UmpStream(m) => O::UmpStream(m.into_owned()),
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
            #[cfg(feature = "flex-data")]
            M::FlexData(m) => O::FlexData(m.into_owned()),
            M::Midi1ChannelVoice(m) => O::Midi1ChannelVoice(m.into_owned()),
            M::Midi2ChannelVoice(m) => O::Midi2ChannelVoice(m.into_owned()),
            M::Utility(m) => O::Utility(m.into_owned()),
            M::UmpStream(m) => O::UmpStream(m.into_owned()),
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
            #[cfg(feature = "flex-data")]
            B::FlexData(m) => M::FlexData(m.into()),
            B::Midi1ChannelVoice(m) => M::Midi1ChannelVoice(m.into()),
            B::Midi2ChannelVoice(m) => M::Midi2ChannelVoice(m.into()),
            B::Utility(m) => M::Utility(m.into()),
            B::UmpStream(m) => M::UmpStream(m.into()),
            B::SystemCommon(m) => M::SystemCommon(m.into()),
            B::Sysex7(m) => M::Sysex7(m.into()),
            B::Sysex8(m) => M::Sysex8(m.into()),
        }
    }
}

impl<'a> core::convert::From<MessageOwned> for Message<'a> {
    fn from(value: MessageOwned) -> Self {
        use Message as M;
        use MessageOwned as O;
        match value {
            #[cfg(feature = "std")]
            #[cfg(feature = "flex-data")]
            O::FlexData(m) => M::FlexData(m.into()),
            O::Midi1ChannelVoice(m) => M::Midi1ChannelVoice(m.into()),
            O::Midi2ChannelVoice(m) => M::Midi2ChannelVoice(m.into()),
            O::Utility(m) => M::Utility(m.into()),
            #[cfg(feature = "std")]
            O::UmpStream(m) => M::UmpStream(m.into()),
            O::SystemCommon(m) => M::SystemCommon(m.into()),
            #[cfg(feature = "std")]
            O::Sysex7(m) => M::Sysex7(m.into()),
            #[cfg(feature = "std")]
            O::Sysex8(m) => M::Sysex8(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> core::convert::From<Sysex8Owned> for Message<'a> {
    fn from(value: Sysex8Owned) -> Self {
        <MessageOwned as core::convert::From<Sysex8Owned>>::from(value).into()
    }
}

#[cfg(feature = "std")]
impl<'a> core::convert::From<Sysex7Owned> for Message<'a> {
    fn from(value: Sysex7Owned) -> Self {
        <MessageOwned as core::convert::From<Sysex7Owned>>::from(value).into()
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

#[cfg(feature = "std")]
macro_rules! from_ump_stream_message_impl {
    ($message: ty) => {
        from_message_impl!($message, UmpStreamOwned);
    };
}

#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::device_identity::DeviceIdentityOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::end_of_clip::EndOfClipOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::endpoint_discovery::EndpointDiscoveryOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::endpoint_info::EndpointInfoOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::endpoint_name::EndpointNameOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::function_block_discovery::FunctionBlockDiscoveryOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::function_block_info::FunctionBlockInfoOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::function_block_name::FunctionBlockNameOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::product_instance_id::ProductInstanceIdOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(ump_stream::start_of_clip::StartOfClipOwned);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(
    ump_stream::stream_configuration_notification::StreamConfigurationNotificationOwned
);
#[cfg(feature = "std")]
from_ump_stream_message_impl!(
    ump_stream::stream_configuration_request::StreamConfigurationRequestOwned
);

#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
macro_rules! from_flex_data_message_impl {
    ($message: ty) => {
        from_message_impl!($message, FlexDataOwned);
    };
}

// from_utility_message_impl!(utility::no_op::NoOpOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::set_chord_name::SetChordNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::set_key_signature::SetKeySignatureOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::set_metronome::SetMetronomeOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::set_tempo::SetTempoOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::set_time_signature::SetTimeSignatureOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::unknown_metadata_text::UnknownMetadataTextOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::project_name::ProjectNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::composition_name::CompositionNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::midi_clip_name::MidiClipNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::copyright_notice::CopyrightNoticeOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::composer_name::ComposerNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::lyricist_name::LyricistNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::arranger_name::ArrangerNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::publisher_name::PublisherNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::primary_performer_name::PrimaryPerformerNameOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(
    flex_data::accompanying_performer_name::AccompanyingPerformerNameOwned
);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::recording_date::RecordingDateOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::recording_location::RecordingLocationOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::unknown_performance_text::UnknownPerformanceTextOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::lyrics::LyricsOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::lyrics_language::LyricsLanguageOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::ruby::RubyOwned);
#[cfg(feature = "std")]
#[cfg(feature = "flex-data")]
from_flex_data_message_impl!(flex_data::ruby_language::RubyLanguageOwned);

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "std")]
    use crate::util::debug;
    use pretty_assertions::assert_eq;

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

    #[cfg(feature = "std")]
    #[test]
    fn ump_stream_builder() {
        assert_eq!(
            debug::Data(
                Message::builder()
                    .ump_stream()
                    .function_block_name()
                    .name("VibratoVanguard: Leading Waves of Euphony🚀🎶🌊")
                    .function_block(0x5)
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0xF412_0556,
                0x6962_7261,
                0x746F_5661,
                0x6E67_7561,
                0xF812_0572,
                0x643A_204C,
                0x6561_6469,
                0x6E67_2057,
                0xF812_0561,
                0x7665_7320,
                0x6F66_2045,
                0x7570_686F,
                0xF812_056E,
                0x79F0_9F9A,
                0x80F0_9F8E,
                0xB6F0_9F8C,
                0xFC12_058A,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn sysex8_builder() {
        assert_eq!(
            debug::Data(
                Message::builder()
                    .sysex8()
                    .payload(0..50)
                    .group(u4::new(0xE))
                    .stream_id(0xBE)
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0x5E1E_BE00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5E2E_BE0D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x5E2E_BE1A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x5E3C_BE27,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ]),
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn sysex7_builder() {
        assert_eq!(
            debug::Data(
                Message::builder()
                    .sysex7()
                    .payload((0..50).into_iter().map(u7::new))
                    .group(u4::new(0xE))
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[
                0x3E16_0001,
                0x0203_0405,
                0x3E26_0607,
                0x0809_0A0B,
                0x3E26_0C0D,
                0x0E0F_1011,
                0x3E26_1213,
                0x1415_1617,
                0x3E26_1819,
                0x1A1B_1C1D,
                0x3E26_1E1F,
                0x2021_2223,
                0x3E26_2425,
                0x2627_2829,
                0x3E26_2A2B,
                0x2C2D_2E2F,
                0x3E32_3031,
                0x0000_0000
            ]),
        );
    }

    #[test]
    fn sysex8_from_data() {
        assert!(Message::from_data(&[
            0x5E1E_BE00,
            0x0102_0304,
            0x0506_0708,
            0x090A_0B0C,
            0x5E2E_BE0D,
            0x0E0F_1011,
            0x1213_1415,
            0x1617_1819,
            0x5E2E_BE1A,
            0x1B1C_1D1E,
            0x1F20_2122,
            0x2324_2526,
            0x5E3C_BE27,
            0x2829_2A2B,
            0x2C2D_2E2F,
            0x3031_0000,
        ])
        .is_ok());
    }

    #[cfg(feature = "std")]
    #[cfg(feature = "flex-data")]
    #[test]
    fn flex_data_builder() {
        assert_eq!(
            debug::Data(
                Message::builder()
                    .flex_data()
                    .composer_name()
                    .group(u4::new(0x4))
                    .text("Tár")
                    .build()
                    .unwrap()
                    .data()
            ),
            debug::Data(&[0xD410_0105, 0x54C3_A172, 0x0000_0000, 0x0000_0000,]),
        );
    }
}
