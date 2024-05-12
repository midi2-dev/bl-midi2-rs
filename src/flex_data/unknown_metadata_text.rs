use crate::{detail::common_properties, flex_data};

const STATUS: u8 = 0x00;

#[midi2_proc::generate_message(Via(crate::flex_data::FlexData), MinSizeUmp(4))]
struct UnknownMetadataText {
    #[property(common_properties::UmpMessageTypeProperty<{flex_data::UMP_MESSAGE_TYPE}>)]
    ump_type: (),

    #[property(flex_data::GroupProperty)]
    group: crate::ux::u4,

    #[property(flex_data::OptionalChannelProperty)]
    optional_channel: Option<crate::ux::u4>,

    #[property(flex_data::BankProperty<{flex_data::METADATA_TEXT_BANK}>)]
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

impl<B: crate::buffer::Ump> crate::traits::Size<B> for UnknownMetadataText<B> {
    fn size(&self) -> usize {
        flex_data::flex_data_dyn_size(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{traits::Grouped, ux::*};
    use pretty_assertions::assert_eq;

    #[test]
    fn text_bytes() {
        let mut message = UnknownMetadataText::<std::vec::Vec<u32>>::new();
        message.set_text("Gimme some signal!");
        let _ = message.text_bytes();
    }

    #[test]
    fn new() {
        assert_eq!(
            UnknownMetadataText::<std::vec::Vec<u32>>::new(),
            UnknownMetadataText(std::vec![0xD010_0100, 0x0, 0x0, 0x0]),
        )
    }

    #[test]
    fn set_text() {
        let mut message = UnknownMetadataText::<std::vec::Vec<u32>>::new();
        message.set_text("Gimme some signal!");
        assert_eq!(
            message,
            UnknownMetadataText(std::vec![
                0xD050_0100,
                0x4769_6D6D,
                0x6520_736F,
                0x6D65_2073,
                0xD0D0_0100,
                0x6967_6E61,
                0x6C21_0000,
                0x0000_0000,
            ])
        )
    }

    #[test]
    fn set_long_text() {
        let mut message = UnknownMetadataText::<std::vec::Vec<u32>>::new();
        message.set_text("Synth wizardry: turning knobs and flipping switches until it sounds like a laser battle in space! 💫🔊🚀");
        assert_eq!(
            message,
            UnknownMetadataText(std::vec![
                0xD050_0100,
                0x5379_6E74,
                0x6820_7769,
                0x7A61_7264,
                0xD090_0100,
                0x7279_3A20,
                0x7475_726E,
                0x696E_6720,
                0xD090_0100,
                0x6B6E_6F62,
                0x7320_616E,
                0x6420_666C,
                0xD090_0100,
                0x6970_7069,
                0x6E67_2073,
                0x7769_7463,
                0xD090_0100,
                0x6865_7320,
                0x756E_7469,
                0x6C20_6974,
                0xD090_0100,
                0x2073_6F75,
                0x6E64_7320,
                0x6C69_6B65,
                0xD090_0100,
                0x2061_206C,
                0x6173_6572,
                0x2062_6174,
                0xD090_0100,
                0x746C_6520,
                0x696E_2073,
                0x7061_6365,
                0xD090_0100,
                0x2120_F09F,
                0x92AB_F09F,
                0x948A_F09F,
                0xD0D0_0100,
                0x9A80_0000,
                0x0000_0000,
                0x0000_0000,
            ])
        )
    }

    #[test]
    fn set_long_text_and_reset_to_short() {
        let mut message = UnknownMetadataText::<std::vec::Vec<u32>>::new();
        message.set_text("Synth wizardry: turning knobs and flipping switches until it sounds like a laser battle in space! 💫🔊🚀");
        message.set_text("Gimme some signal!");
        assert_eq!(
            message,
            UnknownMetadataText(std::vec![
                0xD050_0100,
                0x4769_6D6D,
                0x6520_736F,
                0x6D65_2073,
                0xD0D0_0100,
                0x6967_6E61,
                0x6C21_0000,
                0x0000_0000,
            ])
        )
    }

    #[test]
    fn set_group() {
        let mut message = UnknownMetadataText::<std::vec::Vec<u32>>::new();
        message.set_text("Gimme some signal!");
        message.set_group(u4::new(0xA));
        assert_eq!(
            message,
            UnknownMetadataText(std::vec![
                0xDA50_0100,
                0x4769_6D6D,
                0x6520_736F,
                0x6D65_2073,
                0xDAD0_0100,
                0x6967_6E61,
                0x6C21_0000,
                0x0000_0000,
            ])
        )
    }

    #[test]
    fn try_set_text() {
        let mut message = UnknownMetadataText::<[u32; 8]>::try_new().unwrap();
        message
            .try_set_text("Gimme some signal!")
            .expect("Shouldn't fail");
        assert_eq!(
            message,
            UnknownMetadataText([
                0xD050_0100,
                0x4769_6D6D,
                0x6520_736F,
                0x6D65_2073,
                0xD0D0_0100,
                0x6967_6E61,
                0x6C21_0000,
                0x0000_0000,
            ])
        )
    }

    #[test]
    fn try_from() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Ok(UnknownMetadataText(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            )),
        )
    }

    #[test]
    fn try_from_inconsistent_status() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0101,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Err(crate::error::Error::InvalidData("Incorrect message status")),
        )
    }

    #[test]
    fn try_from_inconsistent_bank() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0200,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Err(crate::error::Error::InvalidData("Incorrect message bank")),
        )
    }

    #[test]
    fn try_from_inconsistent_groups() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xDA10_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xDBD0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Err(crate::error::Error::InvalidData(
                crate::detail::helpers::ERR_INCONSISTENT_GROUPS
            )),
        )
    }

    #[test]
    fn try_from_expected_start() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD010_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Err(crate::error::Error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_BEGIN
            )),
        )
    }

    #[test]
    fn try_from_expected_end() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD090_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            ),
            Err(crate::error::Error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_END
            )),
        )
    }

    #[test]
    fn try_from_expected_complete() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[0xD050_0100, 0x4769_6D6D, 0x6520_736F, 0x6D65_2073,][..]
            ),
            Err(crate::error::Error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_COMPLETE
            )),
        )
    }

    #[test]
    fn try_from_expected_continue() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD090_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                ][..]
            ),
            Err(crate::error::Error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_CONTINUE
            )),
        )
    }

    #[test]
    fn read_text_bytes() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            )
            .unwrap()
            .text_bytes()
            .collect::<std::vec::Vec<u8>>(),
            std::vec![
                0x47, 0x69, 0x6D, 0x6D, 0x65, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x69, 0x67,
                0x6E, 0x61, 0x6C, 0x21,
            ]
        )
    }

    #[test]
    fn read_empty_text_bytes() {
        assert_eq!(
            UnknownMetadataText::<std::vec::Vec<u32>>::new()
                .text_bytes()
                .collect::<std::vec::Vec<u8>>(),
            std::vec![],
        )
    }

    #[test]
    #[cfg(feature = "std")]
    fn read_string() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4769_6D6D,
                    0x6520_736F,
                    0x6D65_2073,
                    0xD0D0_0100,
                    0x6967_6E61,
                    0x6C21_0000,
                    0x0000_0000,
                ][..]
            )
            .unwrap()
            .text(),
            "Gimme some signal!",
        )
    }

    #[test]
    fn set_string_multiple_of_12_length() {
        let mut message = UnknownMetadataText::new();
        message.set_text("Digital Audio Workstation - DAW36-16");
        assert_eq!(
            message,
            UnknownMetadataText(std::vec![
                0xD050_0100,
                0x4469_6769,
                0x7461_6C20,
                0x4175_6469,
                0xD090_0100,
                0x6F20_576F,
                0x726B_7374,
                0x6174_696F,
                0xD0D0_0100,
                0x6E20_2D20,
                0x4441_5733,
                0x362D_3136,
            ])
        );
    }

    #[test]
    fn data() {
        assert_eq!(
            UnknownMetadataText::try_from(
                &[
                    0xD050_0100,
                    0x4469_6769,
                    0x7461_6C20,
                    0x4175_6469,
                    0xD090_0100,
                    0x6F20_576F,
                    0x726B_7374,
                    0x6174_696F,
                    0xD0D0_0100,
                    0x6E20_2D20,
                    0x4441_5733,
                    0x362D_3136,
                ][..]
            )
            .unwrap()
            .data(),
            &[
                0xD050_0100,
                0x4469_6769,
                0x7461_6C20,
                0x4175_6469,
                0xD090_0100,
                0x6F20_576F,
                0x726B_7374,
                0x6174_696F,
                0xD0D0_0100,
                0x6E20_2D20,
                0x4441_5733,
                0x362D_3136,
            ]
        );
    }
}
