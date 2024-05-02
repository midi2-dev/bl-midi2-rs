use crate::{
    buffer::{BufferMut, Ump},
    util::{property::Property, BitOps},
};

// pub mod flex_data_group;

pub mod set_chord_name;
// pub mod set_key_signature;
// pub mod set_metronome;
// pub mod set_tempo;
// pub mod set_time_signature;
// pub mod text;
pub mod tonic;

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

// pub trait FlexData: Data {
//     fn bank(&self) -> Bank {
//         use Bank::*;
//         match (self.data()[0] & 0x0000_FF00) >> 8 {
//             0x0 => SetupAndPerformance,
//             0x1 => MetadataText,
//             0x2 => PerformanceText,
//             _ => panic!(),
//         }
//     }
//     fn status(&self) -> u8 {
//         self.data()[0].octet(3)
//     }
// }

pub struct StatusProperty<const STATUS: u8>;

impl<const STATUS: u8, B: Ump> Property<B> for StatusProperty<STATUS> {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if buffer.buffer()[0].octet(3) == STATUS {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: BufferMut,
    {
        buffer.buffer_mut()[0].set_octet(3, STATUS);
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

pub struct BankProperty<const BANK: u8>;

impl<const BANK: u8, B: Ump> Property<B> for BankProperty<BANK> {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if buffer.buffer()[0].octet(2) == BANK {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message bank"))
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: BufferMut,
    {
        buffer.buffer_mut()[0].set_octet(2, BANK);
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct FormatProperty<const FORMAT: u8>;

impl<const FORMAT: u8, B: Ump> Property<B> for FormatProperty<FORMAT> {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if FORMAT == buffer.buffer()[0].crumb(4).into() {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message format"))
        }
    }
    fn write(buffer: &mut B, _v: Self::Type) -> crate::result::Result<()>
    where
        B: BufferMut,
    {
        buffer.buffer_mut()[0].set_crumb(4, crate::numeric_types::u2::new(FORMAT));
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct OptionalChannelProperty;

impl<B: Ump> Property<B> for OptionalChannelProperty {
    type Type = Option<crate::numeric_types::u4>;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        use crate::numeric_types::u2;
        Ok(if buffer.buffer()[0].crumb(5) == u2::new(0x0) {
            Some(buffer.buffer()[0].nibble(3))
        } else {
            None
        })
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        use crate::numeric_types::u2;
        use crate::numeric_types::u4;
        let data = buffer.buffer_mut();
        match v {
            Some(channel) => {
                data[0].set_crumb(5, u2::new(0x0));
                data[0].set_nibble(3, channel);
            }
            None => {
                data[0].set_crumb(5, u2::new(0x1));
                data[0].set_nibble(3, u4::new(0x0));
            }
        }
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}
// fn bank_from_buffer<B: Ump>(buffer: &B) -> u8 {
//     buffer.buffer()[0].octet(2)
// }
//
// fn bank_to_buffer<B: Ump + BufferMut>(buffer: &B, u8) {
//     buffer.buffer_mut()[0].set_octet(2)
// }

// pub fn channel_from_buffer(buffer: &[u32]) -> Option<u4> {
//     <Ump as Property<Option<u4>, UmpSchema<0x003F_0000, 0x0, 0x0, 0x0>, ()>>::get(buffer)
// }
