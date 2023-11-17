// use crate::{
//     util::{schema::*, BitOps},
//     *,
// };

mod helpers;
pub mod midi1_channel_voice;
pub mod midi2_channel_voice;
pub mod system_common;
// pub mod system_exclusive_7bit;
pub mod system_exclusive_8bit;
pub mod utility;

// pub use midi1_channel_voice::Midi1ChannelVoiceMessage;
// pub use midi2_channel_voice::Midi2ChannelVoiceMessage;
// pub use system_common::SystemCommonMessage;
// pub use system_exclusive_7bit::Sysex7Message;
// pub use system_exclusive_8bit::Sysex8Message;
// pub use utility::UtilityMessage;

// pub enum MidiMessage<'a, B>
// where
//     B: Buffer
//         + Property<
//             NumericalConstant<{ midi1_channel_voice::CHANNEL_PRESSURE_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::CONTROL_CHANGE_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::KEY_PRESSURE_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::NOTE_OFF_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::NOTE_ON_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::PITCH_BEND_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::PROGRAM_CHANGE_CODE }>,
//             UmpSchema<0x00F0_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xF0, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ midi1_channel_voice::TYPE_CODE }>,
//             UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
//             (),
//         > + Property<
//             NumericalConstant<{ system_common::ACTIVE_SENSING }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::CONTINUE }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::RESET }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::SONG_POSITION_POINTER }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::SONG_SELECT }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::START }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::STOP }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::TIME_CODE }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::TIME_CODE }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::TIMING_CLOCK }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::TUNE_REQUEST }>,
//             UmpSchema<0x00FF_0000, 0x0, 0x0, 0x0>,
//             BytesSchema<0xFF, 0x0, 0x0>,
//         > + Property<
//             NumericalConstant<{ system_common::TYPE_CODE }>,
//             UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>,
//             (),
//         > + Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x7F>>
//         + Property<u14, UmpSchema<0x0000_7F7F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x7F>>
//         + Property<u4, UmpSchema<0x000F_0000, 0x0, 0x0, 0x0>, BytesSchema<0x0F, 0x0, 0x0>>
//         + Property<u7, UmpSchema<0x0000_007F, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x0, 0x7F>>
//         + Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>
//         + Property<u7, UmpSchema<0x0000_7F00, 0x0, 0x0, 0x0>, BytesSchema<0x0, 0x7F, 0x0>>,
// {
//     Midi1ChannelVoice(Midi1ChannelVoiceMessage<'a, B>),
//     Midi2ChannelVoice(Midi2ChannelVoiceMessage<'a>),
//     Sysex7(Sysex7Message<'a, B>),
//     Sysex8(Sysex8Message<'a>),
//     Utility(UtilityMessage<'a>),
//     SystemCommon(SystemCommonMessage<'a, B>),
// }
//
// use MidiMessage::*;
//
// const MIDI1_CHANNEL_VOICE_CODE: u8 = 2;
// const MIDI2_CHANNEL_VOICE_CODE: u8 = 4;
// const SYSEX7_CODE: u8 = 3;
// const SYSEX8_CODE: u8 = 5;
// const UTILITY_CODE: u8 = 0;
// const SYSTEM_COMMON_CODE: u8 = 1;
//
// impl<'a> Message<'a, Ump> for MidiMessage<'a, Ump> {
//     fn data(&self) -> &'a [u32] {
//         match self {
//             Midi1ChannelVoice(m) => m.data(),
//             Midi2ChannelVoice(m) => m.data(),
//             Sysex7(m) => m.data(),
//             Sysex8(m) => m.data(),
//             Utility(m) => m.data(),
//             SystemCommon(m) => m.data(),
//         }
//     }
//     fn from_data_unchecked(buffer: &'a [u32]) -> Self {
//         match u8::from(buffer[0].nibble(0)) {
//             MIDI1_CHANNEL_VOICE_CODE => {
//                 Midi1ChannelVoice(Midi1ChannelVoiceMessage::from_data_unchecked(buffer))
//             }
//             MIDI2_CHANNEL_VOICE_CODE => {
//                 Midi2ChannelVoice(Midi2ChannelVoiceMessage::from_data_unchecked(buffer))
//             }
//             SYSEX7_CODE => Sysex7(Sysex7Message::from_data_unchecked(buffer)),
//             SYSEX8_CODE => Sysex8(Sysex8Message::from_data_unchecked(buffer)),
//             UTILITY_CODE => Utility(UtilityMessage::from_data_unchecked(buffer)),
//             SYSTEM_COMMON_CODE => SystemCommon(SystemCommonMessage::from_data_unchecked(buffer)),
//             _ => panic!(),
//         }
//     }
//     fn validate_data(buffer: &'a [u32]) -> Result<()> {
//         match u8::from(buffer[0].nibble(0)) {
//             MIDI1_CHANNEL_VOICE_CODE => Midi1ChannelVoiceMessage::validate_data(buffer),
//             MIDI2_CHANNEL_VOICE_CODE => Midi2ChannelVoiceMessage::validate_data(buffer),
//             SYSEX7_CODE => Sysex7Message::<Ump>::validate_data(buffer),
//             SYSEX8_CODE => Sysex8Message::validate_data(buffer),
//             UTILITY_CODE => UtilityMessage::validate_data(buffer),
//             SYSTEM_COMMON_CODE => SystemCommonMessage::<Ump>::validate_data(buffer),
//             _ => Err(Error::InvalidData),
//         }
//     }
// }
