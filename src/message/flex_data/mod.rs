use crate::{
    buffer::Ump,
    numeric_types::u4,
    traits::Data,
    util::schema::{Property, UmpSchema},
    util::BitOps,
};

pub mod flex_data_group;
pub mod set_chord_name;
pub mod set_key_signature;
pub mod set_metronome;
pub mod set_tempo;
pub mod set_time_signature;
pub mod text;
pub mod tonic;

pub mod unknown_metadata_text {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        UnknownMetadataText,
        UnknownMetadataTextMessage,
        UnknownMetadataTextBorrowed,
        UnknownMetadataTextBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x0
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        UnknownMetadataText,
        UnknownMetadataTextMessage,
        UnknownMetadataTextBorrowed,
        UnknownMetadataTextOwned,
        UnknownMetadataTextBuilder,
        UnknownMetadataTextBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x0
    );
}
pub mod project_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        ProjectName,
        ProjectNameMessage,
        ProjectNameBorrowed,
        ProjectNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x1
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        ProjectName,
        ProjectNameMessage,
        ProjectNameBorrowed,
        ProjectNameOwned,
        ProjectNameBuilder,
        ProjectNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x1
    );
}
pub mod composition_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        CompositionName,
        CompositionNameMessage,
        CompositionNameBorrowed,
        CompositionNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x2
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        CompositionName,
        CompositionNameMessage,
        CompositionNameBorrowed,
        CompositionNameOwned,
        CompositionNameBuilder,
        CompositionNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x2
    );
}
pub mod midi_clip_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        MidiClipName,
        MidiClipNameMessage,
        MidiClipNameBorrowed,
        MidiClipNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x3
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        MidiClipName,
        MidiClipNameMessage,
        MidiClipNameBorrowed,
        MidiClipNameOwned,
        MidiClipNameBuilder,
        MidiClipNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x3
    );
}
pub mod copyright_notice {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        CopyrightNotice,
        CopyrightNoticeMessage,
        CopyrightNoticeBorrowed,
        CopyrightNoticeBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x4
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        CopyrightNotice,
        CopyrightNoticeMessage,
        CopyrightNoticeBorrowed,
        CopyrightNoticeOwned,
        CopyrightNoticeBuilder,
        CopyrightNoticeBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x4
    );
}
pub mod composer_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        ComposerName,
        ComposerNameMessage,
        ComposerNameBorrowed,
        ComposerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x5
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        ComposerName,
        ComposerNameMessage,
        ComposerNameBorrowed,
        ComposerNameOwned,
        ComposerNameBuilder,
        ComposerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x5
    );
}
pub mod lyricist_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        LyricistName,
        LyricistNameMessage,
        LyricistNameBorrowed,
        LyricistNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x6
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        LyricistName,
        LyricistNameMessage,
        LyricistNameBorrowed,
        LyricistNameOwned,
        LyricistNameBuilder,
        LyricistNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x6
    );
}
pub mod arranger_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        ArrangerName,
        ArrangerNameMessage,
        ArrangerNameBorrowed,
        ArrangerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x7
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        ArrangerName,
        ArrangerNameMessage,
        ArrangerNameBorrowed,
        ArrangerNameOwned,
        ArrangerNameBuilder,
        ArrangerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x7
    );
}
pub mod publisher_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        PublisherName,
        PublisherNameMessage,
        PublisherNameBorrowed,
        PublisherNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x8
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        PublisherName,
        PublisherNameMessage,
        PublisherNameBorrowed,
        PublisherNameOwned,
        PublisherNameBuilder,
        PublisherNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x8
    );
}
pub mod primary_performer_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        PrimaryPerformerName,
        PrimaryPerformerNameMessage,
        PrimaryPerformerNameBorrowed,
        PrimaryPerformerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x9
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        PrimaryPerformerName,
        PrimaryPerformerNameMessage,
        PrimaryPerformerNameBorrowed,
        PrimaryPerformerNameOwned,
        PrimaryPerformerNameBuilder,
        PrimaryPerformerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0x9
    );
}
pub mod accompanying_performer_name {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        AccompanyingPerformerName,
        AccompanyingPerformerNameMessage,
        AccompanyingPerformerNameBorrowed,
        AccompanyingPerformerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xA
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        AccompanyingPerformerName,
        AccompanyingPerformerNameMessage,
        AccompanyingPerformerNameBorrowed,
        AccompanyingPerformerNameOwned,
        AccompanyingPerformerNameBuilder,
        AccompanyingPerformerNameBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xA
    );
}
pub mod recording_date {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        RecordingDate,
        RecordingDateMessage,
        RecordingDateBorrowed,
        RecordingDateBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xB
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        RecordingDate,
        RecordingDateMessage,
        RecordingDateBorrowed,
        RecordingDateOwned,
        RecordingDateBuilder,
        RecordingDateBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xB
    );
}
pub mod recording_location {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        RecordingLocation,
        RecordingLocationMessage,
        RecordingLocationBorrowed,
        RecordingLocationBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xC
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        RecordingLocation,
        RecordingLocationMessage,
        RecordingLocationBorrowed,
        RecordingLocationOwned,
        RecordingLocationBuilder,
        RecordingLocationBorrowedBuilder,
        super::METADATA_TEXT_BANK,
        0xC
    );
}
pub mod unknown_performance_text {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        UnknownPerformanceText,
        UnknownPerformanceTextMessage,
        UnknownPerformanceTextBorrowed,
        UnknownPerformanceTextBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x0
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        UnknownPerformanceText,
        UnknownPerformanceTextMessage,
        UnknownPerformanceTextBorrowed,
        UnknownPerformanceTextOwned,
        UnknownPerformanceTextBuilder,
        UnknownPerformanceTextBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x0
    );
}
pub mod lyrics {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        Lyrics,
        LyricsMessage,
        LyricsBorrowed,
        LyricsBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x1
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        Lyrics,
        LyricsMessage,
        LyricsBorrowed,
        LyricsOwned,
        LyricsBuilder,
        LyricsBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x1
    );
}
pub mod lyrics_language {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        LyricsLanguage,
        LyricsLanguageMessage,
        LyricsLanguageBorrowed,
        LyricsLanguageBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x2
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        LyricsLanguage,
        LyricsLanguageMessage,
        LyricsLanguageBorrowed,
        LyricsLanguageOwned,
        LyricsLanguageBuilder,
        LyricsLanguageBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x2
    );
}
pub mod ruby {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        Ruby,
        RubyMessage,
        RubyBorrowed,
        RubyBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x3
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        Ruby,
        RubyMessage,
        RubyBorrowed,
        RubyOwned,
        RubyBuilder,
        RubyBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x3
    );
}
pub mod ruby_language {
    #[cfg(not(feature = "std"))]
    super::text::flex_data_text_message!(
        RubyLanguage,
        RubyLanguageMessage,
        RubyLanguageBorrowed,
        RubyLanguageBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x4
    );
    #[cfg(feature = "std")]
    super::text::flex_data_text_message_std!(
        RubyLanguage,
        RubyLanguageMessage,
        RubyLanguageBorrowed,
        RubyLanguageOwned,
        RubyLanguageBuilder,
        RubyLanguageBorrowedBuilder,
        super::PERFORMANCE_TEXT_BANK,
        0x4
    );
}

const TYPE_CODE: u32 = 0xD;
const SETUP_AND_PERFORMANCE_BANK: u32 = 0x0;
const METADATA_TEXT_BANK: u8 = 0x1;
const PERFORMANCE_TEXT_BANK: u8 = 0x2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bank {
    SetupAndPerformance,
    MetadataText,
    PerformanceText,
}

pub trait FlexData: Data {
    fn bank(&self) -> Bank {
        use Bank::*;
        match (self.data()[0] & 0x0000_FF00) >> 8 {
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

pub fn status_from_buffer(buffer: &[u32]) -> u8 {
    buffer[0].octet(3)
}

pub fn bank_from_buffer(buffer: &[u32]) -> u8 {
    buffer[0].octet(2)
}

pub fn channel_from_buffer(buffer: &[u32]) -> Option<u4> {
    <Ump as Property<Option<u4>, UmpSchema<0x003F_0000, 0x0, 0x0, 0x0>, ()>>::get(buffer)
}
