use crate::{
    buffer::{BufferMut, Ump},
    detail::{
        property::{Property, ReadProperty, WriteProperty},
        BitOps,
    },
};

mod text;

mod set_chord_name;
mod set_key_signature;
mod set_metronome;
mod set_tempo;
mod set_time_signature;
mod tonic;
mod unknown_metadata_text;
mod project_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x1;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct ProjectName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for ProjectName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for ProjectName<B> {}
}
mod composition_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x2;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct CompositionName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for CompositionName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for CompositionName<B> {}
}
mod midi_clip_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x3;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct MidiClipName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for MidiClipName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for MidiClipName<B> {}
}
mod copyright_notice {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x4;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct CopyrightNotice {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for CopyrightNotice<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for CopyrightNotice<B> {}
}
mod composer_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x5;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct ComposerName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        name: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        name_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        name: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for ComposerName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for ComposerName<B> {}
}
mod lyricist_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x6;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct LyricistName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for LyricistName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for LyricistName<B> {}
}
mod arranger_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x7;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct ArrangerName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for ArrangerName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for ArrangerName<B> {}
}
mod publisher_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x8;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct PublisherName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for PublisherName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for PublisherName<B> {}
}
mod primary_performer_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0x9;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct PrimaryPerformerName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for PrimaryPerformerName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for PrimaryPerformerName<B> {}
}
mod accompanying_performer_name {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0xA;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct AccompanyingPerformerName {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for AccompanyingPerformerName<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for AccompanyingPerformerName<B> {}
}
mod recording_date {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0xB;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct RecordingDate {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for RecordingDate<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for RecordingDate<B> {}
}
mod recording_location {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::METADATA_TEXT_BANK;
    const STATUS: u8 = 0xC;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct RecordingLocation {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for RecordingLocation<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for RecordingLocation<B> {}
}
mod unknown_performance_text {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::PERFORMANCE_TEXT_BANK;
    const STATUS: u8 = 0x0;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct UnknownPerformanceText {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for UnknownPerformanceText<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for UnknownPerformanceText<B> {}
}
mod lyrics {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::PERFORMANCE_TEXT_BANK;
    const STATUS: u8 = 0x1;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct Lyrics {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for Lyrics<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for Lyrics<B> {}
}
mod lyrics_language {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::PERFORMANCE_TEXT_BANK;
    const STATUS: u8 = 0x2;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct LyricsLanguage {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for LyricsLanguage<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for LyricsLanguage<B> {}
}
mod ruby {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::PERFORMANCE_TEXT_BANK;
    const STATUS: u8 = 0x3;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct Ruby {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for Ruby<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for Ruby<B> {}
}
mod ruby_language {
    use crate::{detail::common_properties, flex_data};

    const BANK: u8 = super::PERFORMANCE_TEXT_BANK;
    const STATUS: u8 = 0x4;

    #[midi2_proc::generate_message(MinSizeUmp(4))]
    struct RubyLanguage {
        #[property(crate::utility::JitterReductionProperty)]
        jitter_reduction: Option<crate::utility::JitterReduction>,
        #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
        ump_type: (),
        #[property(flex_data::GroupProperty)]
        group: crate::ux::u4,
        #[property(flex_data::OptionalChannelProperty)]
        optional_channel: Option<crate::ux::u4>,
        #[property(flex_data::BankProperty<BANK>)]
        bank: (),
        #[property(flex_data::StatusProperty<{STATUS}>)]
        status: (),
        #[property(flex_data::ConsistentFormatsProperty)]
        #[readonly]
        consisten_formats: (),
        #[property(flex_data::text::TextWriteStrProperty)]
        #[writeonly]
        #[resize]
        text: &str,
        #[property(flex_data::text::TextReadBytesProperty)]
        #[readonly]
        text_bytes: flex_data::text::TextBytesIterator,
        #[property(flex_data::text::TextReadStringProperty)]
        #[readonly]
        #[std]
        text: std::string::String,
    }

    impl<B: crate::buffer::Ump> crate::traits::Size<B> for RubyLanguage<B> {
        fn size(&self) -> usize {
            flex_data::flex_data_dyn_size(&self.0)
        }
    }
    impl<B: crate::buffer::Ump> flex_data::FlexDataMessage<B> for RubyLanguage<B> {}
}

pub use accompanying_performer_name::*;
pub use arranger_name::*;
pub use composer_name::*;
pub use composition_name::*;
pub use copyright_notice::*;
pub use lyricist_name::*;
pub use lyrics::*;
pub use lyrics_language::*;
pub use midi_clip_name::*;
pub use primary_performer_name::*;
pub use project_name::*;
pub use publisher_name::*;
pub use recording_date::*;
pub use recording_location::*;
pub use ruby::*;
pub use ruby_language::*;
pub use set_chord_name::{
    Alteration, ChordType, SetChordName, SharpsFlats as SetChordNameSharpsFlats,
};
pub use set_key_signature::{SetKeySignature, SharpsFlats as SetKeySignatureSharpsFlats};
pub use set_metronome::*;
pub use set_tempo::*;
pub use set_time_signature::*;
pub use tonic::*;
pub use unknown_metadata_text::*;
pub use unknown_performance_text::*;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0xD;
pub(crate) const COMPLETE_FORMAT: u8 = 0x0;
pub(crate) const START_FORMAT: u8 = 0x1;
pub(crate) const CONTINUE_FORMAT: u8 = 0x2;
pub(crate) const END_FORMAT: u8 = 0x3;
pub(crate) const SETUP_AND_PERFORMANCE_BANK: u8 = 0x0;
pub(crate) const METADATA_TEXT_BANK: u8 = 0x1;
pub(crate) const PERFORMANCE_TEXT_BANK: u8 = 0x2;

#[derive(
    derive_more::From,
    midi2_proc::Data,
    midi2_proc::Grouped,
    midi2_proc::RebufferFrom,
    midi2_proc::TryRebufferFrom,
    midi2_proc::JitterReduced,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum FlexData<B: crate::buffer::Ump> {
    SetChordName(set_chord_name::SetChordName<B>),
    SetKeySignature(set_key_signature::SetKeySignature<B>),
    SetMetronome(set_metronome::SetMetronome<B>),
    SetTempo(set_tempo::SetTempo<B>),
    SetTimeSignature(set_time_signature::SetTimeSignature<B>),
    UnknownMetadataText(unknown_metadata_text::UnknownMetadataText<B>),
    ProjectName(project_name::ProjectName<B>),
    CompositionName(composition_name::CompositionName<B>),
    MidiClipName(midi_clip_name::MidiClipName<B>),
    CopyrightNotice(copyright_notice::CopyrightNotice<B>),
    ComposerName(composer_name::ComposerName<B>),
    LyricistName(lyricist_name::LyricistName<B>),
    ArrangerName(arranger_name::ArrangerName<B>),
    PublisherName(publisher_name::PublisherName<B>),
    PrimaryPerformerName(primary_performer_name::PrimaryPerformerName<B>),
    AccompanyingPerformerName(accompanying_performer_name::AccompanyingPerformerName<B>),
    RecordingDate(recording_date::RecordingDate<B>),
    RecordingLocation(recording_location::RecordingLocation<B>),
    UnknownPerformanceText(unknown_performance_text::UnknownPerformanceText<B>),
    Lyrics(lyrics::Lyrics<B>),
    LyricsLanguage(lyrics_language::LyricsLanguage<B>),
    Ruby(ruby::Ruby<B>),
    RubyLanguage(ruby_language::RubyLanguage<B>),
}

impl<'a> TryFrom<&'a [u32]> for FlexData<&'a [u32]> {
    type Error = crate::error::Error;
    fn try_from(value: &'a [u32]) -> Result<Self, Self::Error> {
        use crate::buffer::UmpPrivate;
        use FlexData::*;
        if value.message().len() < 1 {
            return Err(crate::error::Error::InvalidData("Slice is too short"));
        };
        Ok(match value.message()[0].word(1) {
            0x00_00 => SetTempo(set_tempo::SetTempo::try_from(value)?.into()),
            0x00_01 => {
                SetTimeSignature(set_time_signature::SetTimeSignature::try_from(value)?.into())
            }
            0x00_02 => SetMetronome(set_metronome::SetMetronome::try_from(value)?.into()),
            0x00_03 => SetKeySignature(set_key_signature::SetKeySignature::try_from(value)?.into()),
            0x00_04 => SetChordName(set_chord_name::SetChordName::try_from(value)?.into()),
            0x01_00 => UnknownMetadataText(
                unknown_metadata_text::UnknownMetadataText::try_from(value)?.into(),
            ),
            0x01_01 => ProjectName(project_name::ProjectName::try_from(value)?.into()),
            0x01_02 => CompositionName(composition_name::CompositionName::try_from(value)?.into()),
            0x01_03 => MidiClipName(midi_clip_name::MidiClipName::try_from(value)?.into()),
            0x01_04 => CopyrightNotice(copyright_notice::CopyrightNotice::try_from(value)?.into()),
            0x01_05 => ComposerName(composer_name::ComposerName::try_from(value)?.into()),
            0x01_06 => LyricistName(lyricist_name::LyricistName::try_from(value)?.into()),
            0x01_07 => ArrangerName(arranger_name::ArrangerName::try_from(value)?.into()),
            0x01_08 => PublisherName(publisher_name::PublisherName::try_from(value)?.into()),
            0x01_09 => PrimaryPerformerName(
                primary_performer_name::PrimaryPerformerName::try_from(value)?.into(),
            ),
            0x01_10 => AccompanyingPerformerName(
                accompanying_performer_name::AccompanyingPerformerName::try_from(value)?.into(),
            ),
            0x01_11 => RecordingDate(recording_date::RecordingDate::try_from(value)?.into()),
            0x01_12 => {
                RecordingLocation(recording_location::RecordingLocation::try_from(value)?.into())
            }
            0x02_00 => UnknownPerformanceText(
                unknown_performance_text::UnknownPerformanceText::try_from(value)?.into(),
            ),
            0x02_01 => Lyrics(lyrics::Lyrics::try_from(value)?.into()),
            0x02_02 => LyricsLanguage(lyrics_language::LyricsLanguage::try_from(value)?.into()),
            0x02_03 => Ruby(ruby::Ruby::try_from(value)?.into()),
            0x02_04 => RubyLanguage(ruby_language::RubyLanguage::try_from(value)?.into()),
            _ => Err(crate::error::Error::InvalidData(
                "Couldn't interpret flex data status / bank fields",
            ))?,
        })
    }
}

impl<B: Ump> FlexDataMessage<B> for FlexData<B> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bank {
    SetupAndPerformance,
    MetadataText,
    PerformanceText,
}

pub trait FlexDataMessage<B: crate::buffer::Ump>: crate::traits::Data<B> {
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
            Err(crate::error::Error::InvalidData("Incorrect message status"))
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
            Err(crate::error::Error::InvalidData("Incorrect message bank"))
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
            Err(crate::error::Error::InvalidData("Incorrect message format"))
        }
    }
}

impl<const FORMAT: u8, B: Ump + BufferMut> WriteProperty<B> for FormatProperty<FORMAT> {
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        buffer.buffer_mut().message_mut()[0].set_crumb(4, crate::ux::u2::new(FORMAT));
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
    type Type = Option<crate::ux::u4>;
}

impl<'a, B: Ump> ReadProperty<'a, B> for OptionalChannelProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        optional_channel_from_slice(buffer.buffer().message())
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

fn optional_channel_to_slice(data: &mut [u32], channel: Option<crate::ux::u4>) {
    use crate::ux::{u2, u4};
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

fn optional_channel_from_slice(data: &[u32]) -> Option<ux::u4> {
    if data[0].crumb(5) == ux::u2::new(0x0) {
        Some(data[0].nibble(3))
    } else {
        None
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
        use crate::ux::u2;
        if buffer.buffer().message()[0].crumb(5) != u2::new(0x0) {
            Ok(())
        } else {
            Err(crate::error::Error::InvalidData(
                "Address field should be non zero.",
            ))
        }
    }
}

impl<B: Ump + BufferMut> WriteProperty<B> for NoChannelProperty {
    fn write(buffer: &mut B, _: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::ux::u2;
        use crate::ux::u4;

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
        use crate::detail::helpers::validate_sysex_group_statuses;

        validate_sysex_group_statuses(
            buffer.buffer().message(),
            |p| u8::from(p[0].crumb(4)) == COMPLETE_FORMAT,
            |p| u8::from(p[0].crumb(4)) == START_FORMAT,
            |p| u8::from(p[0].crumb(4)) == CONTINUE_FORMAT,
            |p| u8::from(p[0].crumb(4)) == END_FORMAT,
            4,
            crate::ux::u4::new(UMP_MESSAGE_TYPE),
        )
    }
}

struct GroupProperty;

impl<B: Ump> Property<B> for GroupProperty {
    type Type = crate::ux::u4;
}

impl<'a, B: Ump> ReadProperty<'a, B> for GroupProperty {
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        buffer.buffer().message()[0].nibble(1)
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::detail::helpers::sysex_group_consistent_groups;
        sysex_group_consistent_groups(
            buffer.buffer().message(),
            4,
            crate::ux::u4::new(UMP_MESSAGE_TYPE),
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
            let status: u8 = p[0].crumb(4).into();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_empty() {
        let buffer = [];
        assert_eq!(
            FlexData::try_from(&buffer[..]),
            Err(crate::error::Error::InvalidData("Slice is too short")),
        );
    }

    #[test]
    fn try_from_text() {
        let buffer = [
            0x0000_0000_u32,
            0xD050_0106,
            0x4769_6D6D,
            0x6520_736F,
            0x6D65_2073,
            0xD0D0_0106,
            0x6967_6E61,
            0x6C21_0000,
            0x0000_0000,
        ];
        assert_eq!(
            FlexData::try_from(&buffer[..]),
            Ok(FlexData::LyricistName(
                lyricist_name::LyricistName::try_from(&buffer[..]).unwrap()
            ))
        );
    }

    #[test]
    fn try_from_set_tempo() {
        let buffer = [0xD710_0000_u32, 0xF751_FE05];
        assert_eq!(
            FlexData::try_from(&buffer[..]),
            Ok(FlexData::SetTempo(
                set_tempo::SetTempo::try_from(&buffer[..]).unwrap()
            ))
        );
    }

    #[test]
    fn read_bank() {
        assert_eq!(
            FlexData::try_from(&[0xD710_0000_u32, 0xF751_FE05][..])
                .unwrap()
                .bank(),
            Bank::SetupAndPerformance,
        );
    }

    #[test]
    fn set_jr() {
        use crate::traits::{JitterReduced, RebufferInto};
        use crate::utility::JitterReduction;

        let mut message: FlexData<std::vec::Vec<u32>> =
            FlexData::try_from(&[0xD710_0000_u32, 0xF751_FE05][..])
                .unwrap()
                .rebuffer_into();

        let jr = Some(JitterReduction::Timestamp(0x1234));
        message.set_jitter_reduction(jr.clone());

        assert_eq!(message.jitter_reduction(), jr.clone());
    }
}
