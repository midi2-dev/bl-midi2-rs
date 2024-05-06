mod common_properties;
mod helpers;

#[cfg(feature = "midi1-channel-voice")]
pub mod channel_voice1;
#[cfg(feature = "midi2-channel-voice")]
pub mod channel_voice2;
#[cfg(feature = "flex-data")]
pub mod flex_data;
#[cfg(feature = "sysex7")]
pub mod sysex7;
#[cfg(feature = "sysex8")]
pub mod sysex8;
#[cfg(feature = "system-common")]
pub mod system_common;
#[cfg(feature = "ump-stream")]
pub mod ump_stream;
pub mod utility;

// #[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
// #[non_exhaustive]
// pub enum Message<'a> {
//     #[cfg(feature = "flex-data")]
//     FlexData(FlexDataMessage<'a>),
//     #[cfg(feature = "midi1-channel-voice")]
//     ChannelVoice1(ChannelVoice1Message<'a>),
//     #[cfg(feature = "midi2-channel-voice")]
//     ChannelVoice1(ChannelVoice1Message<'a>),
//     #[cfg(feature = "sysex7")]
//     Sysex7(Sysex7Message<'a>),
//     #[cfg(feature = "sysex8")]
//     Sysex8(Sysex8Message<'a>),
//     #[cfg(feature = "system-common")]
//     SystemCommon(SystemCommonMessage<'a>),
//     #[cfg(feature = "ump-stream")]
//     UmpStream(UmpStreamMessage<'a>),
//     #[cfg(feature = "utility")]
//     Utility(UtilityMessage<'a>),
// }

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // #[cfg(feature = "midi1-channel-voice")]
    // fn from_byte_data() {
    //     use pretty_assertions::assert_eq;
    //     assert_eq!(
    //         Message::from_byte_data(&[0xAB, 0x60, 0x33]),
    //         Message::builder()
    //             .channel_voice1()
    //             .key_pressure()
    //             .channel(u4::new(0xB))
    //             .note(u7::new(0x60))
    //             .pressure(u7::new(0x33))
    //             .build(),
    //     );
    // }

    // #[cfg(feature = "std")]
    // #[cfg(feature = "ump-stream")]
    // #[test]
    // fn ump_stream_builder() {
    //     use crate::test_support::debug;
    //     use pretty_assertions::assert_eq;
    //     assert_eq!(
    //         debug::Data(
    //             Message::builder()
    //                 .ump_stream()
    //                 .function_block_name()
    //                 .name("VibratoVanguard: Leading Waves of Euphony🚀🎶🌊")
    //                 .function_block(0x5)
    //                 .build()
    //                 .unwrap()
    //                 .data()
    //         ),
    //         debug::Data(&[
    //             0xF412_0556,
    //             0x6962_7261,
    //             0x746F_5661,
    //             0x6E67_7561,
    //             0xF812_0572,
    //             0x643A_204C,
    //             0x6561_6469,
    //             0x6E67_2057,
    //             0xF812_0561,
    //             0x7665_7320,
    //             0x6F66_2045,
    //             0x7570_686F,
    //             0xF812_056E,
    //             0x79F0_9F9A,
    //             0x80F0_9F8E,
    //             0xB6F0_9F8C,
    //             0xFC12_058A,
    //             0x0000_0000,
    //             0x0000_0000,
    //             0x0000_0000,
    //         ]),
    //     );
    // }

    // #[cfg(feature = "std")]
    // #[cfg(feature = "sysex8")]
    // #[test]
    // fn sysex8_builder() {
    //     use crate::test_support::debug;
    //     use pretty_assertions::assert_eq;
    //     assert_eq!(
    //         debug::Data(
    //             Message::builder()
    //                 .sysex8()
    //                 .payload(0..50)
    //                 .group(u4::new(0xE))
    //                 .stream_id(0xBE)
    //                 .build()
    //                 .unwrap()
    //                 .data()
    //         ),
    //         debug::Data(&[
    //             0x5E1E_BE00,
    //             0x0102_0304,
    //             0x0506_0708,
    //             0x090A_0B0C,
    //             0x5E2E_BE0D,
    //             0x0E0F_1011,
    //             0x1213_1415,
    //             0x1617_1819,
    //             0x5E2E_BE1A,
    //             0x1B1C_1D1E,
    //             0x1F20_2122,
    //             0x2324_2526,
    //             0x5E3C_BE27,
    //             0x2829_2A2B,
    //             0x2C2D_2E2F,
    //             0x3031_0000,
    //         ]),
    //     );
    // }

    // #[cfg(feature = "std")]
    // #[cfg(feature = "sysex7")]
    // #[test]
    // fn sysex7_builder() {
    //     use crate::test_support::debug;
    //     use pretty_assertions::assert_eq;
    //     assert_eq!(
    //         debug::Data(
    //             Message::builder()
    //                 .sysex7()
    //                 .payload((0..50).into_iter().map(u7::new))
    //                 .group(u4::new(0xE))
    //                 .build()
    //                 .unwrap()
    //                 .data()
    //         ),
    //         debug::Data(&[
    //             0x3E16_0001,
    //             0x0203_0405,
    //             0x3E26_0607,
    //             0x0809_0A0B,
    //             0x3E26_0C0D,
    //             0x0E0F_1011,
    //             0x3E26_1213,
    //             0x1415_1617,
    //             0x3E26_1819,
    //             0x1A1B_1C1D,
    //             0x3E26_1E1F,
    //             0x2021_2223,
    //             0x3E26_2425,
    //             0x2627_2829,
    //             0x3E26_2A2B,
    //             0x2C2D_2E2F,
    //             0x3E32_3031,
    //             0x0000_0000
    //         ]),
    //     );
    // }

    // #[test]
    // #[cfg(feature = "sysex8")]
    // fn sysex8_from_data() {
    //     assert!(Message::from_data(&[
    //         0x5E1E_BE00,
    //         0x0102_0304,
    //         0x0506_0708,
    //         0x090A_0B0C,
    //         0x5E2E_BE0D,
    //         0x0E0F_1011,
    //         0x1213_1415,
    //         0x1617_1819,
    //         0x5E2E_BE1A,
    //         0x1B1C_1D1E,
    //         0x1F20_2122,
    //         0x2324_2526,
    //         0x5E3C_BE27,
    //         0x2829_2A2B,
    //         0x2C2D_2E2F,
    //         0x3031_0000,
    //     ])
    //     .is_ok());
    // }

    // #[cfg(feature = "std")]
    // #[cfg(feature = "flex-data")]
    // #[test]
    // fn flex_data_builder() {
    //     use crate::test_support::debug;
    //     use pretty_assertions::assert_eq;
    //     assert_eq!(
    //         debug::Data(
    //             Message::builder()
    //                 .flex_data()
    //                 .composer_name()
    //                 .group(u4::new(0x4))
    //                 .text("Tár")
    //                 .build()
    //                 .unwrap()
    //                 .data()
    //         ),
    //         debug::Data(&[0xD410_0105, 0x54C3_A172, 0x0000_0000, 0x0000_0000,]),
    //     );
    // }
}
