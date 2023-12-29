#[cfg(feature = "std")]
use crate::traits::{IntoOwned, Level2Message};
use crate::{
    buffer::Ump,
    numeric_types::u4,
    traits::{Data, FromData},
    util::schema::{Property, UmpSchema},
    util::BitOps,
    Error, Result,
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

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FlexDataMessage<'a> {
    SetChordName(set_chord_name::SetChordNameMessage<'a>),
    SetKeySignature(set_key_signature::SetKeySignatureMessage<'a>),
    SetMetronome(set_metronome::SetMetronomeMessage<'a>),
    SetTempo(set_tempo::SetTempoMessage<'a>),
    SetTimeSignature(set_time_signature::SetTimeSignatureMessage<'a>),
    UnknownMetadataText(unknown_metadata_text::UnknownMetadataTextMessage<'a>),
    ProjectName(project_name::ProjectNameMessage<'a>),
    CompositionName(composition_name::CompositionNameMessage<'a>),
    MidiClipName(midi_clip_name::MidiClipNameMessage<'a>),
    CopyrightNotice(copyright_notice::CopyrightNoticeMessage<'a>),
    ComposerName(composer_name::ComposerNameMessage<'a>),
    LyricistName(lyricist_name::LyricistNameMessage<'a>),
    ArrangerName(arranger_name::ArrangerNameMessage<'a>),
    PublisherName(publisher_name::PublisherNameMessage<'a>),
    PrimaryPerformerName(primary_performer_name::PrimaryPerformerNameMessage<'a>),
    AccompanyingPerformerName(accompanying_performer_name::AccompanyingPerformerNameMessage<'a>),
    RecordingDate(recording_date::RecordingDateMessage<'a>),
    RecordingLocation(recording_location::RecordingLocationMessage<'a>),
    UnknownPerformanceText(unknown_performance_text::UnknownPerformanceTextMessage<'a>),
    Lyrics(lyrics::LyricsMessage<'a>),
    LyricsLanguage(lyrics_language::LyricsLanguageMessage<'a>),
    Ruby(ruby::RubyMessage<'a>),
    RubyLanguage(ruby_language::RubyLanguageMessage<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FlexDataBorrowed<'a> {
    SetChordName(set_chord_name::SetChordNameBorrowed<'a>),
    SetKeySignature(set_key_signature::SetKeySignatureBorrowed<'a>),
    SetMetronome(set_metronome::SetMetronomeBorrowed<'a>),
    SetTempo(set_tempo::SetTempoBorrowed<'a>),
    SetTimeSignature(set_time_signature::SetTimeSignatureBorrowed<'a>),
    UnknownMetadataText(unknown_metadata_text::UnknownMetadataTextBorrowed<'a>),
    ProjectName(project_name::ProjectNameBorrowed<'a>),
    CompositionName(composition_name::CompositionNameBorrowed<'a>),
    MidiClipName(midi_clip_name::MidiClipNameBorrowed<'a>),
    CopyrightNotice(copyright_notice::CopyrightNoticeBorrowed<'a>),
    ComposerName(composer_name::ComposerNameBorrowed<'a>),
    LyricistName(lyricist_name::LyricistNameBorrowed<'a>),
    ArrangerName(arranger_name::ArrangerNameBorrowed<'a>),
    PublisherName(publisher_name::PublisherNameBorrowed<'a>),
    PrimaryPerformerName(primary_performer_name::PrimaryPerformerNameBorrowed<'a>),
    AccompanyingPerformerName(accompanying_performer_name::AccompanyingPerformerNameBorrowed<'a>),
    RecordingDate(recording_date::RecordingDateBorrowed<'a>),
    RecordingLocation(recording_location::RecordingLocationBorrowed<'a>),
    UnknownPerformanceText(unknown_performance_text::UnknownPerformanceTextBorrowed<'a>),
    Lyrics(lyrics::LyricsBorrowed<'a>),
    LyricsLanguage(lyrics_language::LyricsLanguageBorrowed<'a>),
    Ruby(ruby::RubyBorrowed<'a>),
    RubyLanguage(ruby_language::RubyLanguageBorrowed<'a>),
}

#[derive(derive_more::From, midi2_attr::Data, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[cfg(feature = "std")]
pub enum FlexDataOwned {
    SetChordName(set_chord_name::SetChordNameOwned),
    SetKeySignature(set_key_signature::SetKeySignatureOwned),
    SetMetronome(set_metronome::SetMetronomeOwned),
    SetTempo(set_tempo::SetTempoOwned),
    SetTimeSignature(set_time_signature::SetTimeSignatureOwned),
    UnknownMetadataText(unknown_metadata_text::UnknownMetadataTextOwned),
    ProjectName(project_name::ProjectNameOwned),
    CompositionName(composition_name::CompositionNameOwned),
    MidiClipName(midi_clip_name::MidiClipNameOwned),
    CopyrightNotice(copyright_notice::CopyrightNoticeOwned),
    ComposerName(composer_name::ComposerNameOwned),
    LyricistName(lyricist_name::LyricistNameOwned),
    ArrangerName(arranger_name::ArrangerNameOwned),
    PublisherName(publisher_name::PublisherNameOwned),
    PrimaryPerformerName(primary_performer_name::PrimaryPerformerNameOwned),
    AccompanyingPerformerName(accompanying_performer_name::AccompanyingPerformerNameOwned),
    RecordingDate(recording_date::RecordingDateOwned),
    RecordingLocation(recording_location::RecordingLocationOwned),
    UnknownPerformanceText(unknown_performance_text::UnknownPerformanceTextOwned),
    Lyrics(lyrics::LyricsOwned),
    LyricsLanguage(lyrics_language::LyricsLanguageOwned),
    Ruby(ruby::RubyOwned),
    RubyLanguage(ruby_language::RubyLanguageOwned),
}

#[derive(Default)]
#[cfg(feature = "std")]
pub struct FlexDataBuilder<M>(core::marker::PhantomData<M>)
where
    M: core::convert::From<set_chord_name::SetChordNameOwned>
        + core::convert::From<set_key_signature::SetKeySignatureOwned>
        + core::convert::From<set_metronome::SetMetronomeOwned>
        + core::convert::From<set_tempo::SetTempoOwned>
        + core::convert::From<set_time_signature::SetTimeSignatureOwned>
        + core::convert::From<unknown_metadata_text::UnknownMetadataTextOwned>
        + core::convert::From<project_name::ProjectNameOwned>
        + core::convert::From<composition_name::CompositionNameOwned>
        + core::convert::From<midi_clip_name::MidiClipNameOwned>
        + core::convert::From<copyright_notice::CopyrightNoticeOwned>
        + core::convert::From<composer_name::ComposerNameOwned>
        + core::convert::From<lyricist_name::LyricistNameOwned>
        + core::convert::From<arranger_name::ArrangerNameOwned>
        + core::convert::From<publisher_name::PublisherNameOwned>
        + core::convert::From<primary_performer_name::PrimaryPerformerNameOwned>
        + core::convert::From<accompanying_performer_name::AccompanyingPerformerNameOwned>
        + core::convert::From<recording_date::RecordingDateOwned>
        + core::convert::From<recording_location::RecordingLocationOwned>
        + core::convert::From<unknown_performance_text::UnknownPerformanceTextOwned>
        + core::convert::From<lyrics::LyricsOwned>
        + core::convert::From<lyrics_language::LyricsLanguageOwned>
        + core::convert::From<ruby::RubyOwned>
        + core::convert::From<ruby_language::RubyLanguageOwned>;

#[cfg(feature = "std")]
impl<M> FlexDataBuilder<M>
where
    M: core::convert::From<set_chord_name::SetChordNameOwned>
        + core::convert::From<set_key_signature::SetKeySignatureOwned>
        + core::convert::From<set_metronome::SetMetronomeOwned>
        + core::convert::From<set_tempo::SetTempoOwned>
        + core::convert::From<set_time_signature::SetTimeSignatureOwned>
        + core::convert::From<unknown_metadata_text::UnknownMetadataTextOwned>
        + core::convert::From<project_name::ProjectNameOwned>
        + core::convert::From<composition_name::CompositionNameOwned>
        + core::convert::From<midi_clip_name::MidiClipNameOwned>
        + core::convert::From<copyright_notice::CopyrightNoticeOwned>
        + core::convert::From<composer_name::ComposerNameOwned>
        + core::convert::From<lyricist_name::LyricistNameOwned>
        + core::convert::From<arranger_name::ArrangerNameOwned>
        + core::convert::From<publisher_name::PublisherNameOwned>
        + core::convert::From<primary_performer_name::PrimaryPerformerNameOwned>
        + core::convert::From<accompanying_performer_name::AccompanyingPerformerNameOwned>
        + core::convert::From<recording_date::RecordingDateOwned>
        + core::convert::From<recording_location::RecordingLocationOwned>
        + core::convert::From<unknown_performance_text::UnknownPerformanceTextOwned>
        + core::convert::From<lyrics::LyricsOwned>
        + core::convert::From<lyrics_language::LyricsLanguageOwned>
        + core::convert::From<ruby::RubyOwned>
        + core::convert::From<ruby_language::RubyLanguageOwned>,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn set_chord_name(self) -> set_chord_name::SetChordNameBuilder<M> {
        set_chord_name::SetChordNameBuilder::new()
    }
    pub fn set_key_signature(self) -> set_key_signature::SetKeySignatureBuilder<M> {
        set_key_signature::SetKeySignatureBuilder::new()
    }
    pub fn set_metronome(self) -> set_metronome::SetMetronomeBuilder<M> {
        set_metronome::SetMetronomeBuilder::new()
    }
    pub fn set_tempo(self) -> set_tempo::SetTempoBuilder<M> {
        set_tempo::SetTempoBuilder::new()
    }
    pub fn set_time_signature(self) -> set_time_signature::SetTimeSignatureBuilder<M> {
        set_time_signature::SetTimeSignatureBuilder::new()
    }
    pub fn unknown_metadata_text(self) -> unknown_metadata_text::UnknownMetadataTextBuilder<M> {
        unknown_metadata_text::UnknownMetadataTextBuilder::new()
    }
    pub fn project_name(self) -> project_name::ProjectNameBuilder<M> {
        project_name::ProjectNameBuilder::new()
    }
    pub fn composition_name(self) -> composition_name::CompositionNameBuilder<M> {
        composition_name::CompositionNameBuilder::new()
    }
    pub fn midi_clip_name(self) -> midi_clip_name::MidiClipNameBuilder<M> {
        midi_clip_name::MidiClipNameBuilder::new()
    }
    pub fn copyright_notice(self) -> copyright_notice::CopyrightNoticeBuilder<M> {
        copyright_notice::CopyrightNoticeBuilder::new()
    }
    pub fn composer_name(self) -> composer_name::ComposerNameBuilder<M> {
        composer_name::ComposerNameBuilder::new()
    }
    pub fn lyricist_name(self) -> lyricist_name::LyricistNameBuilder<M> {
        lyricist_name::LyricistNameBuilder::new()
    }
    pub fn arranger_name(self) -> arranger_name::ArrangerNameBuilder<M> {
        arranger_name::ArrangerNameBuilder::new()
    }
    pub fn publisher_name(self) -> publisher_name::PublisherNameBuilder<M> {
        publisher_name::PublisherNameBuilder::new()
    }
    pub fn primary_performer_name(self) -> primary_performer_name::PrimaryPerformerNameBuilder<M> {
        primary_performer_name::PrimaryPerformerNameBuilder::new()
    }
    pub fn accompanying_performer_name(
        self,
    ) -> accompanying_performer_name::AccompanyingPerformerNameBuilder<M> {
        accompanying_performer_name::AccompanyingPerformerNameBuilder::new()
    }
    pub fn recording_date(self) -> recording_date::RecordingDateBuilder<M> {
        recording_date::RecordingDateBuilder::new()
    }
    pub fn recording_location(self) -> recording_location::RecordingLocationBuilder<M> {
        recording_location::RecordingLocationBuilder::new()
    }
    pub fn unknown_performance_text(
        self,
    ) -> unknown_performance_text::UnknownPerformanceTextBuilder<M> {
        unknown_performance_text::UnknownPerformanceTextBuilder::new()
    }
    pub fn lyrics(self) -> lyrics::LyricsBuilder<M> {
        lyrics::LyricsBuilder::new()
    }
    pub fn lyrics_language(self) -> lyrics_language::LyricsLanguageBuilder<M> {
        lyrics_language::LyricsLanguageBuilder::new()
    }
    pub fn ruby(self) -> ruby::RubyBuilder<M> {
        ruby::RubyBuilder::new()
    }
    pub fn ruby_language(self) -> ruby_language::RubyLanguageBuilder<M> {
        ruby_language::RubyLanguageBuilder::new()
    }
}

#[cfg(feature = "std")]
impl<'a> FlexDataMessage<'a> {
    pub fn builder() -> FlexDataBuilder<Self> {
        FlexDataBuilder::new()
    }
}

#[cfg(feature = "std")]
impl FlexDataOwned {
    pub fn builder() -> FlexDataBuilder<Self> {
        FlexDataBuilder::new()
    }
}

impl<'a> core::convert::From<FlexDataBorrowed<'a>> for FlexDataMessage<'a> {
    fn from(value: FlexDataBorrowed<'a>) -> Self {
        use FlexDataBorrowed as B;
        use FlexDataMessage as M;
        match value {
            B::SetChordName(m) => M::SetChordName(m.into()),
            B::SetKeySignature(m) => M::SetKeySignature(m.into()),
            B::SetMetronome(m) => M::SetMetronome(m.into()),
            B::SetTempo(m) => M::SetTempo(m.into()),
            B::SetTimeSignature(m) => M::SetTimeSignature(m.into()),
            B::UnknownMetadataText(m) => M::UnknownMetadataText(m.into()),
            B::ProjectName(m) => M::ProjectName(m.into()),
            B::CompositionName(m) => M::CompositionName(m.into()),
            B::MidiClipName(m) => M::MidiClipName(m.into()),
            B::CopyrightNotice(m) => M::CopyrightNotice(m.into()),
            B::ComposerName(m) => M::ComposerName(m.into()),
            B::LyricistName(m) => M::LyricistName(m.into()),
            B::ArrangerName(m) => M::ArrangerName(m.into()),
            B::PublisherName(m) => M::PublisherName(m.into()),
            B::PrimaryPerformerName(m) => M::PrimaryPerformerName(m.into()),
            B::AccompanyingPerformerName(m) => M::AccompanyingPerformerName(m.into()),
            B::RecordingDate(m) => M::RecordingDate(m.into()),
            B::RecordingLocation(m) => M::RecordingLocation(m.into()),
            B::UnknownPerformanceText(m) => M::UnknownPerformanceText(m.into()),
            B::Lyrics(m) => M::Lyrics(m.into()),
            B::LyricsLanguage(m) => M::LyricsLanguage(m.into()),
            B::Ruby(m) => M::Ruby(m.into()),
            B::RubyLanguage(m) => M::RubyLanguage(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> core::convert::From<FlexDataOwned> for FlexDataMessage<'a> {
    fn from(value: FlexDataOwned) -> Self {
        use FlexDataMessage as M;
        use FlexDataOwned as O;
        match value {
            O::SetChordName(m) => M::SetChordName(m.into()),
            O::SetKeySignature(m) => M::SetKeySignature(m.into()),
            O::SetMetronome(m) => M::SetMetronome(m.into()),
            O::SetTempo(m) => M::SetTempo(m.into()),
            O::SetTimeSignature(m) => M::SetTimeSignature(m.into()),
            O::UnknownMetadataText(m) => M::UnknownMetadataText(m.into()),
            O::ProjectName(m) => M::ProjectName(m.into()),
            O::CompositionName(m) => M::CompositionName(m.into()),
            O::MidiClipName(m) => M::MidiClipName(m.into()),
            O::CopyrightNotice(m) => M::CopyrightNotice(m.into()),
            O::ComposerName(m) => M::ComposerName(m.into()),
            O::LyricistName(m) => M::LyricistName(m.into()),
            O::ArrangerName(m) => M::ArrangerName(m.into()),
            O::PublisherName(m) => M::PublisherName(m.into()),
            O::PrimaryPerformerName(m) => M::PrimaryPerformerName(m.into()),
            O::AccompanyingPerformerName(m) => M::AccompanyingPerformerName(m.into()),
            O::RecordingDate(m) => M::RecordingDate(m.into()),
            O::RecordingLocation(m) => M::RecordingLocation(m.into()),
            O::UnknownPerformanceText(m) => M::UnknownPerformanceText(m.into()),
            O::Lyrics(m) => M::Lyrics(m.into()),
            O::LyricsLanguage(m) => M::LyricsLanguage(m.into()),
            O::Ruby(m) => M::Ruby(m.into()),
            O::RubyLanguage(m) => M::RubyLanguage(m.into()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a, M> core::convert::From<M> for FlexDataMessage<'a>
where
    M: Level2Message,
    FlexDataOwned: core::convert::From<M>,
{
    fn from(value: M) -> Self {
        <FlexDataOwned as core::convert::From<M>>::from(value).into()
    }
}

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

impl<'a> FromData<'a> for FlexDataBorrowed<'a> {
    type Target = Self;
    fn validate_data(data: &[u32]) -> Result<()> {
        match data[0].word(1) {
            0x00_00 => set_tempo::SetTempoBorrowed::validate_data(data),
            0x00_01 => set_time_signature::SetTimeSignatureBorrowed::validate_data(data),
            0x00_02 => set_metronome::SetMetronomeBorrowed::validate_data(data),
            0x00_03 => set_key_signature::SetKeySignatureBorrowed::validate_data(data),
            0x00_04 => set_chord_name::SetChordNameBorrowed::validate_data(data),
            0x01_00 => unknown_metadata_text::UnknownMetadataTextBorrowed::validate_data(data),
            0x01_01 => project_name::ProjectNameBorrowed::validate_data(data),
            0x01_02 => composition_name::CompositionNameBorrowed::validate_data(data),
            0x01_03 => midi_clip_name::MidiClipNameBorrowed::validate_data(data),
            0x01_04 => copyright_notice::CopyrightNoticeBorrowed::validate_data(data),
            0x01_05 => composer_name::ComposerNameBorrowed::validate_data(data),
            0x01_06 => lyricist_name::LyricistNameBorrowed::validate_data(data),
            0x01_07 => arranger_name::ArrangerNameBorrowed::validate_data(data),
            0x01_08 => publisher_name::PublisherNameBorrowed::validate_data(data),
            0x01_09 => primary_performer_name::PrimaryPerformerNameBorrowed::validate_data(data),
            0x01_10 => {
                accompanying_performer_name::AccompanyingPerformerNameBorrowed::validate_data(data)
            }
            0x01_11 => recording_date::RecordingDateBorrowed::validate_data(data),
            0x01_12 => recording_location::RecordingLocationBorrowed::validate_data(data),
            0x02_00 => {
                unknown_performance_text::UnknownPerformanceTextBorrowed::validate_data(data)
            }
            0x02_01 => lyrics::LyricsBorrowed::validate_data(data),
            0x02_02 => lyrics_language::LyricsLanguageBorrowed::validate_data(data),
            0x02_03 => ruby::RubyBorrowed::validate_data(data),
            0x02_04 => ruby_language::RubyLanguageBorrowed::validate_data(data),
            _ => Err(Error::InvalidData),
        }
    }
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        use FlexDataBorrowed::*;
        match data[0].word(1) {
            0x00_00 => SetTempo(set_tempo::SetTempoBorrowed::from_data_unchecked(data)),
            0x00_01 => SetTimeSignature(
                set_time_signature::SetTimeSignatureBorrowed::from_data_unchecked(data),
            ),
            0x00_02 => SetMetronome(set_metronome::SetMetronomeBorrowed::from_data_unchecked(
                data,
            )),
            0x00_03 => SetKeySignature(
                set_key_signature::SetKeySignatureBorrowed::from_data_unchecked(data),
            ),
            0x00_04 => SetChordName(set_chord_name::SetChordNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_00 => UnknownMetadataText(
                unknown_metadata_text::UnknownMetadataTextBorrowed::from_data_unchecked(data),
            ),
            0x01_01 => ProjectName(project_name::ProjectNameBorrowed::from_data_unchecked(data)),
            0x01_02 => CompositionName(
                composition_name::CompositionNameBorrowed::from_data_unchecked(data),
            ),
            0x01_03 => MidiClipName(midi_clip_name::MidiClipNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_04 => CopyrightNotice(
                copyright_notice::CopyrightNoticeBorrowed::from_data_unchecked(data),
            ),
            0x01_05 => ComposerName(composer_name::ComposerNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_06 => LyricistName(lyricist_name::LyricistNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_07 => ArrangerName(arranger_name::ArrangerNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_08 => PublisherName(publisher_name::PublisherNameBorrowed::from_data_unchecked(
                data,
            )),
            0x01_09 => PrimaryPerformerName(
                primary_performer_name::PrimaryPerformerNameBorrowed::from_data_unchecked(data),
            ),
            0x01_10 => AccompanyingPerformerName(
                accompanying_performer_name::AccompanyingPerformerNameBorrowed::from_data_unchecked(
                    data,
                ),
            ),
            0x01_11 => RecordingDate(recording_date::RecordingDateBorrowed::from_data_unchecked(
                data,
            )),
            0x01_12 => RecordingLocation(
                recording_location::RecordingLocationBorrowed::from_data_unchecked(data),
            ),
            0x02_00 => UnknownPerformanceText(
                unknown_performance_text::UnknownPerformanceTextBorrowed::from_data_unchecked(data),
            ),
            0x02_01 => Lyrics(lyrics::LyricsBorrowed::from_data_unchecked(data)),
            0x02_02 => {
                LyricsLanguage(lyrics_language::LyricsLanguageBorrowed::from_data_unchecked(data))
            }
            0x02_03 => Ruby(ruby::RubyBorrowed::from_data_unchecked(data)),
            0x02_04 => RubyLanguage(ruby_language::RubyLanguageBorrowed::from_data_unchecked(
                data,
            )),
            _ => panic!(),
        }
    }
}

impl<'a> FromData<'a> for FlexDataMessage<'a> {
    type Target = Self;
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        FlexDataBorrowed::validate_data(buffer)
    }
    fn from_data_unchecked(buffer: &'a [u32]) -> Self::Target {
        FlexDataBorrowed::from_data_unchecked(buffer).into()
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for FlexDataBorrowed<'a> {
    type Owned = FlexDataOwned;
    fn into_owned(self) -> Self::Owned {
        use FlexDataBorrowed as B;
        use FlexDataOwned as O;
        match self {
            B::SetChordName(m) => O::SetChordName(m.into_owned()),
            B::SetKeySignature(m) => O::SetKeySignature(m.into_owned()),
            B::SetMetronome(m) => O::SetMetronome(m.into_owned()),
            B::SetTempo(m) => O::SetTempo(m.into_owned()),
            B::SetTimeSignature(m) => O::SetTimeSignature(m.into_owned()),
            B::UnknownMetadataText(m) => O::UnknownMetadataText(m.into_owned()),
            B::ProjectName(m) => O::ProjectName(m.into_owned()),
            B::CompositionName(m) => O::CompositionName(m.into_owned()),
            B::MidiClipName(m) => O::MidiClipName(m.into_owned()),
            B::CopyrightNotice(m) => O::CopyrightNotice(m.into_owned()),
            B::ComposerName(m) => O::ComposerName(m.into_owned()),
            B::LyricistName(m) => O::LyricistName(m.into_owned()),
            B::ArrangerName(m) => O::ArrangerName(m.into_owned()),
            B::PublisherName(m) => O::PublisherName(m.into_owned()),
            B::PrimaryPerformerName(m) => O::PrimaryPerformerName(m.into_owned()),
            B::AccompanyingPerformerName(m) => O::AccompanyingPerformerName(m.into_owned()),
            B::RecordingDate(m) => O::RecordingDate(m.into_owned()),
            B::RecordingLocation(m) => O::RecordingLocation(m.into_owned()),
            B::UnknownPerformanceText(m) => O::UnknownPerformanceText(m.into_owned()),
            B::Lyrics(m) => O::Lyrics(m.into_owned()),
            B::LyricsLanguage(m) => O::LyricsLanguage(m.into_owned()),
            B::Ruby(m) => O::Ruby(m.into_owned()),
            B::RubyLanguage(m) => O::RubyLanguage(m.into_owned()),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> IntoOwned for FlexDataMessage<'a> {
    type Owned = FlexDataOwned;
    fn into_owned(self) -> FlexDataOwned {
        use FlexDataMessage as M;
        use FlexDataOwned as O;
        match self {
            M::SetChordName(m) => O::SetChordName(m.into_owned()),
            M::SetKeySignature(m) => O::SetKeySignature(m.into_owned()),
            M::SetMetronome(m) => O::SetMetronome(m.into_owned()),
            M::SetTempo(m) => O::SetTempo(m.into_owned()),
            M::SetTimeSignature(m) => O::SetTimeSignature(m.into_owned()),
            M::UnknownMetadataText(m) => O::UnknownMetadataText(m.into_owned()),
            M::ProjectName(m) => O::ProjectName(m.into_owned()),
            M::CompositionName(m) => O::CompositionName(m.into_owned()),
            M::MidiClipName(m) => O::MidiClipName(m.into_owned()),
            M::CopyrightNotice(m) => O::CopyrightNotice(m.into_owned()),
            M::ComposerName(m) => O::ComposerName(m.into_owned()),
            M::LyricistName(m) => O::LyricistName(m.into_owned()),
            M::ArrangerName(m) => O::ArrangerName(m.into_owned()),
            M::PublisherName(m) => O::PublisherName(m.into_owned()),
            M::PrimaryPerformerName(m) => O::PrimaryPerformerName(m.into_owned()),
            M::AccompanyingPerformerName(m) => O::AccompanyingPerformerName(m.into_owned()),
            M::RecordingDate(m) => O::RecordingDate(m.into_owned()),
            M::RecordingLocation(m) => O::RecordingLocation(m.into_owned()),
            M::UnknownPerformanceText(m) => O::UnknownPerformanceText(m.into_owned()),
            M::Lyrics(m) => O::Lyrics(m.into_owned()),
            M::LyricsLanguage(m) => O::LyricsLanguage(m.into_owned()),
            M::Ruby(m) => O::Ruby(m.into_owned()),
            M::RubyLanguage(m) => O::RubyLanguage(m.into_owned()),
        }
    }
}

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
