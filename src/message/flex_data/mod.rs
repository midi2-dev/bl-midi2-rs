use crate::{
    buffer::{BufferMut, Ump},
    util::{
        property::{Property, ReadProperty, WriteProperty},
        BitOps,
    },
};

// pub mod flex_data_group;
mod text;

pub mod set_chord_name;
pub mod set_key_signature;
pub mod set_metronome;
pub mod set_tempo;
pub mod set_time_signature;
pub mod tonic;
pub mod unknown_metadata_text;

const UMP_MESSAGE_TYPE: u8 = 0xD;
const COMPLETE_FORMAT: u8 = 0x0;
const START_FORMAT: u8 = 0x1;
const CONTINUE_FORMAT: u8 = 0x2;
const END_FORMAT: u8 = 0x3;
const SETUP_AND_PERFORMANCE_BANK: u8 = 0x0;
const METADATA_TEXT_BANK: u8 = 0x1;
const PERFORMANCE_TEXT_BANK: u8 = 0x2;

// #[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
// #[non_exhaustive]
// pub enum FlexDataMessage<'a> {
//     SetChordName(set_chord_name::SetChordNameMessage<'a>),
//     SetKeySignature(set_key_signature::SetKeySignatureMessage<'a>),
//     SetMetronome(set_metronome::SetMetronomeMessage<'a>),
//     SetTempo(set_tempo::SetTempoMessage<'a>),
//     SetTimeSignature(set_time_signature::SetTimeSignatureMessage<'a>),
//     UnknownMetadataText(unknown_metadata_text::UnknownMetadataTextMessage<'a>),
//     ProjectName(project_name::ProjectNameMessage<'a>),
//     CompositionName(composition_name::CompositionNameMessage<'a>),
//     MidiClipName(midi_clip_name::MidiClipNameMessage<'a>),
//     CopyrightNotice(copyright_notice::CopyrightNoticeMessage<'a>),
//     ComposerName(composer_name::ComposerNameMessage<'a>),
//     LyricistName(lyricist_name::LyricistNameMessage<'a>),
//     ArrangerName(arranger_name::ArrangerNameMessage<'a>),
//     PublisherName(publisher_name::PublisherNameMessage<'a>),
//     PrimaryPerformerName(primary_performer_name::PrimaryPerformerNameMessage<'a>),
//     AccompanyingPerformerName(accompanying_performer_name::AccompanyingPerformerNameMessage<'a>),
//     RecordingDate(recording_date::RecordingDateMessage<'a>),
//     RecordingLocation(recording_location::RecordingLocationMessage<'a>),
//     UnknownPerformanceText(unknown_performance_text::UnknownPerformanceTextMessage<'a>),
//     Lyrics(lyrics::LyricsMessage<'a>),
//     LyricsLanguage(lyrics_language::LyricsLanguageMessage<'a>),
//     Ruby(ruby::RubyMessage<'a>),
//     RubyLanguage(ruby_language::RubyLanguageMessage<'a>),
// }
//
// const DEVICE_IDENTITY: u32 = 0x2;
// const END_OF_CLIP: u32 = 0x21;
// const ENDPOINT_DISCOVERY: u32 = 0x0;
// const ENDPOINT_INFO: u32 = 0x1;
// const ENDPOINT_NAME: u32 = 0x3;
// const FUNCTION_BLOCK_DISCOVERY: u32 = 0x10;
// const FUNCTION_BLOCK_INFO: u32 = 0x11;
// const FUNCTION_BLOCK_NAME: u32 = 0x12;
// const PRODUCT_INSTANCE_ID: u32 = 0x4;
// const START_OF_CLIP: u32 = 0x20;
// const STREAM_CONFIGURATION_NOTIFICATION: u32 = 0x06;
// const STREAM_CONFIGURATION_REQUEST: u32 = 0x05;

// impl<'a> FromData<'a> for FlexDataBorrowed<'a> {
//     type Target = Self;
//     fn from_data_unchecked(data: &'a [u32]) -> Self {
//         use FlexDataBorrowed::*;
//         match data[0].word(1) {
//             0x00_00 => SetTempo(set_tempo::SetTempoBorrowed::from_data_unchecked(data)),
//             0x00_01 => SetTimeSignature(
//                 set_time_signature::SetTimeSignatureBorrowed::from_data_unchecked(data),
//             ),
//             0x00_02 => SetMetronome(set_metronome::SetMetronomeBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x00_03 => SetKeySignature(
//                 set_key_signature::SetKeySignatureBorrowed::from_data_unchecked(data),
//             ),
//             0x00_04 => SetChordName(set_chord_name::SetChordNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_00 => UnknownMetadataText(
//                 unknown_metadata_text::UnknownMetadataTextBorrowed::from_data_unchecked(data),
//             ),
//             0x01_01 => ProjectName(project_name::ProjectNameBorrowed::from_data_unchecked(data)),
//             0x01_02 => CompositionName(
//                 composition_name::CompositionNameBorrowed::from_data_unchecked(data),
//             ),
//             0x01_03 => MidiClipName(midi_clip_name::MidiClipNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_04 => CopyrightNotice(
//                 copyright_notice::CopyrightNoticeBorrowed::from_data_unchecked(data),
//             ),
//             0x01_05 => ComposerName(composer_name::ComposerNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_06 => LyricistName(lyricist_name::LyricistNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_07 => ArrangerName(arranger_name::ArrangerNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_08 => PublisherName(publisher_name::PublisherNameBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_09 => PrimaryPerformerName(
//                 primary_performer_name::PrimaryPerformerNameBorrowed::from_data_unchecked(data),
//             ),
//             0x01_10 => AccompanyingPerformerName(
//                 accompanying_performer_name::AccompanyingPerformerNameBorrowed::from_data_unchecked(
//                     data,
//                 ),
//             ),
//             0x01_11 => RecordingDate(recording_date::RecordingDateBorrowed::from_data_unchecked(
//                 data,
//             )),
//             0x01_12 => RecordingLocation(
//                 recording_location::RecordingLocationBorrowed::from_data_unchecked(data),
//             ),
//             0x02_00 => UnknownPerformanceText(
//                 unknown_performance_text::UnknownPerformanceTextBorrowed::from_data_unchecked(data),
//             ),
//             0x02_01 => Lyrics(lyrics::LyricsBorrowed::from_data_unchecked(data)),
//             0x02_02 => {
//                 LyricsLanguage(lyrics_language::LyricsLanguageBorrowed::from_data_unchecked(data))
//             }
//             0x02_03 => Ruby(ruby::RubyBorrowed::from_data_unchecked(data)),
//             0x02_04 => RubyLanguage(ruby_language::RubyLanguageBorrowed::from_data_unchecked(
//                 data,
//             )),
//             _ => panic!(),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bank {
    SetupAndPerformance,
    MetadataText,
    PerformanceText,
}

pub trait FlexData<B: crate::buffer::Ump>: crate::traits::Data<B> {
    fn bank(&self) -> Bank {
        use crate::buffer::UmpPrivate;
        use Bank::*;
        match (self.data().message()[0] & 0x0000_FF00) >> 8 {
            0x0 => SetupAndPerformance,
            0x1 => MetadataText,
            0x2 => PerformanceText,
            _ => panic!(),
        }
    }
    fn status(&self) -> u8 {
        self.data()[0].octet(3)
    }
}

pub struct StatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: Ump> Property<B> for StatusProperty<STATUS> {
    type Type = ();
}

impl<'a, const STATUS: u8, B: Ump> ReadProperty<'a, B> for StatusProperty<STATUS> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        if buffer
            .buffer()
            .message()
            .chunks_exact(4)
            .all(|packet| packet[0].octet(3) == STATUS)
        {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
}

impl<const STATUS: u8, B: Ump + BufferMut> WriteProperty<B> for StatusProperty<STATUS> {
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        buffer.buffer_mut().message_mut()[0].set_octet(3, STATUS);
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct BankProperty<const BANK: u8>;

impl<const BANK: u8, B: Ump> Property<B> for BankProperty<BANK> {
    type Type = ();
}

impl<'a, const BANK: u8, B: Ump> ReadProperty<'a, B> for BankProperty<BANK> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        if buffer
            .buffer()
            .message()
            .chunks_exact(4)
            .all(|packet| packet[0].octet(2) == BANK)
        {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message bank"))
        }
    }
}

impl<const BANK: u8, B: Ump + BufferMut> WriteProperty<B> for BankProperty<BANK> {
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        buffer.buffer_mut().message_mut()[0].set_octet(2, BANK);
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct FormatProperty<const FORMAT: u8>;

impl<const FORMAT: u8, B: Ump> Property<B> for FormatProperty<FORMAT> {
    type Type = ();
}

impl<'a, const FORMAT: u8, B: Ump> ReadProperty<'a, B> for FormatProperty<FORMAT> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        if FORMAT == buffer.buffer().message()[0].crumb(4).into() {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message format"))
        }
    }
}

impl<const FORMAT: u8, B: Ump + BufferMut> WriteProperty<B> for FormatProperty<FORMAT> {
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        buffer.buffer_mut().message_mut()[0].set_crumb(4, crate::numeric_types::u2::new(FORMAT));
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct OptionalChannelProperty;

impl<B: Ump> Property<B> for OptionalChannelProperty {
    type Type = Option<crate::numeric_types::u4>;
}

impl<'a, B: Ump> ReadProperty<'a, B> for OptionalChannelProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use crate::numeric_types::u2;
        if buffer.buffer().message()[0].crumb(5) == u2::new(0x0) {
            Some(buffer.buffer()[0].nibble(3))
        } else {
            None
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<B: Ump + BufferMut> WriteProperty<B> for OptionalChannelProperty {
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;

        let buffer_slice = buffer.buffer_mut();
        let data = buffer_slice.message_mut();
        optional_channel_to_slice(data, v);
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

fn optional_channel_to_slice(data: &mut [u32], channel: Option<crate::numeric_types::u4>) {
    use crate::numeric_types::{u2, u4};
    match channel {
        Some(channel) => {
            data[0].set_crumb(5, u2::new(0x0));
            data[0].set_nibble(3, channel);
        }
        None => {
            data[0].set_crumb(5, u2::new(0x1));
            data[0].set_nibble(3, u4::new(0x0));
        }
    }
}

struct NoChannelProperty;

impl<B: Ump> Property<B> for NoChannelProperty {
    type Type = ();
}

impl<'a, B: Ump> ReadProperty<'a, B> for NoChannelProperty {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::numeric_types::u2;
        if buffer.buffer().message()[0].crumb(5) != u2::new(0x0) {
            Ok(())
        } else {
            Err(crate::Error::InvalidData(
                "Address field should be non zero.",
            ))
        }
    }
}

impl<B: Ump + BufferMut> WriteProperty<B> for NoChannelProperty {
    fn write(buffer: &mut B, _: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::numeric_types::u2;
        use crate::numeric_types::u4;

        let buffer_slice = buffer.buffer_mut();
        let data = buffer_slice.message_mut();
        data[0].set_crumb(5, u2::new(0x1));
        data[0].set_nibble(3, u4::new(0x0));
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

struct ConsistentFormatsProperty;

impl<B: Ump> Property<B> for ConsistentFormatsProperty {
    type Type = ();
}

impl<'a, B: Ump> ReadProperty<'a, B> for ConsistentFormatsProperty {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }

    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::message::helpers::validate_sysex_group_statuses;

        validate_sysex_group_statuses(
            buffer.buffer().message(),
            |p| u8::from(p[0].crumb(4)) == COMPLETE_FORMAT,
            |p| u8::from(p[0].crumb(4)) == START_FORMAT,
            |p| u8::from(p[0].crumb(4)) == CONTINUE_FORMAT,
            |p| u8::from(p[0].crumb(4)) == END_FORMAT,
            4,
            crate::numeric_types::u4::new(UMP_MESSAGE_TYPE),
        )
    }
}

struct GroupProperty;

impl<B: Ump> Property<B> for GroupProperty {
    type Type = crate::numeric_types::u4;
}

impl<'a, B: Ump> ReadProperty<'a, B> for GroupProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        buffer.buffer().message()[0].nibble(1)
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::message::helpers::sysex_group_consistent_groups;
        sysex_group_consistent_groups(
            buffer.buffer().message(),
            4,
            crate::numeric_types::u4::new(UMP_MESSAGE_TYPE),
        )
    }
}

impl<B: Ump + BufferMut> WriteProperty<B> for GroupProperty {
    fn write(buffer: &mut B, group: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        for packet in buffer
            .buffer_mut()
            .message_mut()
            .chunks_exact_mut(4)
            .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
        {
            packet[0].set_nibble(1, group);
        }
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

fn flex_data_dyn_size<B: crate::buffer::Ump>(buffer: &B) -> usize {
    use crate::buffer::UmpPrivate;
    let jr_offset = buffer.buffer().jitter_reduction().len();
    buffer
        .buffer()
        .message()
        .chunks_exact(4)
        .position(|p| {
            let status: u8 = p[0].nibble(2).into();
            status == COMPLETE_FORMAT || status == END_FORMAT
        })
        .expect("Message is in an invalid state. Couldn't find end packet.")
        * 4
        + 4
        + jr_offset
}

fn bank_from_buffer(buffer: &[u32]) -> u8 {
    buffer[0].octet(2)
}

fn status_from_buffer(buffer: &[u32]) -> u8 {
    buffer[0].octet(3)
}

fn bank_to_buffer(buffer: &mut [u32], bank: u8) {
    buffer[0].set_octet(2, bank);
}

fn status_to_buffer(buffer: &mut [u32], status: u8) {
    buffer[0].set_octet(3, status);
}

fn clear_payload(buffer: &mut [u32]) {
    for packet in buffer.chunks_exact_mut(4) {
        packet[1] = 0x0;
        packet[2] = 0x0;
        packet[3] = 0x0;
    }
}

// pub fn channel_from_buffer(buffer: &[u32]) -> Option<u4> {
//     <Ump as Property<Option<u4>, UmpSchema<0x003F_0000, 0x0, 0x0, 0x0>, ()>>::get(buffer)
// }
