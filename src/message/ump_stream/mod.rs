use crate::{
    buffer::{BufferMut, Ump},
    util::property,
};

mod device_identity;
mod end_of_clip;
mod endpoint_discovery;
mod endpoint_info;
mod endpoint_name;
mod function_block_discovery;
mod function_block_info;
// pub mod function_block_name;
// pub mod product_instance_id;
mod start_of_clip;
mod stream_configuration_notification;
mod stream_configuration_request;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0xF;
const COMPLETE_FORMAT: u8 = 0x0;
const START_FORMAT: u8 = 0x1;
const CONTINUE_FORMAT: u8 = 0x2;
const END_FORMAT: u8 = 0x3;

// #[derive(midi2_proc::UmpDebug, derive_more::From, midi2_proc::Data, Clone, PartialEq, Eq)]
// #[non_exhaustive]
// pub enum UmpStreamMessage<'a> {
//     DeviceIdentity(DeviceIdentityMessage<'a>),
//     EndOfClip(EndOfClipMessage<'a>),
//     EndpointDiscovery(EndpointDiscoveryMessage<'a>),
//     EndpointInfo(EndpointInfoMessage<'a>),
//     EndpointName(EndpointNameMessage<'a>),
//     FunctionBlockDiscovery(FunctionBlockDiscoveryMessage<'a>),
//     FunctionBlockInfo(FunctionBlockInfoMessage<'a>),
//     FunctionBlockName(FunctionBlockNameMessage<'a>),
//     ProductInstanceId(ProductInstanceIdMessage<'a>),
//     StartOfClip(StartOfClipMessage<'a>),
//     StreamConfigurationNotification(StreamConfigurationNotificationMessage<'a>),
//     StreamConfigurationRequest(StreamConfigurationRequestMessage<'a>),
// }

// impl<'a> FromData<'a> for UmpStreamBorrowed<'a> {
//     type Target = Self;
//     fn validate_data(data: &[u32]) -> Result<()> {
//         let code = (data[0] & 0x03FF_0000) >> 16;
//         match code {
//             DEVICE_IDENTITY => DeviceIdentityBorrowed::validate_data(data),
//             END_OF_CLIP => EndOfClipBorrowed::validate_data(data),
//             ENDPOINT_DISCOVERY => EndpointDiscoveryBorrowed::validate_data(data),
//             ENDPOINT_INFO => EndpointInfoBorrowed::validate_data(data),
//             ENDPOINT_NAME => EndpointNameBorrowed::validate_data(data),
//             FUNCTION_BLOCK_DISCOVERY => FunctionBlockDiscoveryBorrowed::validate_data(data),
//             FUNCTION_BLOCK_INFO => FunctionBlockInfoBorrowed::validate_data(data),
//             FUNCTION_BLOCK_NAME => FunctionBlockNameBorrowed::validate_data(data),
//             PRODUCT_INSTANCE_ID => ProductInstanceIdBorrowed::validate_data(data),
//             START_OF_CLIP => StartOfClipBorrowed::validate_data(data),
//             STREAM_CONFIGURATION_NOTIFICATION => {
//                 StreamConfigurationNotificationBorrowed::validate_data(data)
//             }
//             STREAM_CONFIGURATION_REQUEST => StreamConfigurationRequestBorrowed::validate_data(data),
//             _ => Err(Error::InvalidData),
//         }
//     }
//     fn from_data_unchecked(data: &'a [u32]) -> Self {
//         use UmpStreamBorrowed::*;
//         let code = (data[0] & 0x03FF_0000) >> 16;
//         match code {
//             DEVICE_IDENTITY => DeviceIdentity(DeviceIdentityBorrowed::from_data_unchecked(data)),
//             END_OF_CLIP => EndOfClip(EndOfClipBorrowed::from_data_unchecked(data)),
//             ENDPOINT_DISCOVERY => {
//                 EndpointDiscovery(EndpointDiscoveryBorrowed::from_data_unchecked(data))
//             }
//             ENDPOINT_INFO => EndpointInfo(EndpointInfoBorrowed::from_data_unchecked(data)),
//             ENDPOINT_NAME => EndpointName(EndpointNameBorrowed::from_data_unchecked(data)),
//             FUNCTION_BLOCK_DISCOVERY => {
//                 FunctionBlockDiscovery(FunctionBlockDiscoveryBorrowed::from_data_unchecked(data))
//             }
//             FUNCTION_BLOCK_INFO => {
//                 FunctionBlockInfo(FunctionBlockInfoBorrowed::from_data_unchecked(data))
//             }
//             FUNCTION_BLOCK_NAME => {
//                 FunctionBlockName(FunctionBlockNameBorrowed::from_data_unchecked(data))
//             }
//             PRODUCT_INSTANCE_ID => {
//                 ProductInstanceId(ProductInstanceIdBorrowed::from_data_unchecked(data))
//             }
//             START_OF_CLIP => StartOfClip(StartOfClipBorrowed::from_data_unchecked(data)),
//             STREAM_CONFIGURATION_NOTIFICATION => StreamConfigurationNotification(
//                 StreamConfigurationNotificationBorrowed::from_data_unchecked(data),
//             ),
//             STREAM_CONFIGURATION_REQUEST => StreamConfigurationRequest(
//                 StreamConfigurationRequestBorrowed::from_data_unchecked(data),
//             ),
//             _ => panic!(),
//         }
//     }
// }

struct StatusProperty<const STATUS: u16>;

impl<const STATUS: u16, B: Ump> property::Property<B> for StatusProperty<STATUS> {
    type Type = ();
}

impl<'a, const STATUS: u16, B: Ump> property::ReadProperty<'a, B> for StatusProperty<STATUS> {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::numeric_types::u10;

        if buffer
            .buffer()
            .message()
            .chunks_exact(4)
            .all(|packet| status_from_buffer(packet) == u10::new(STATUS))
        {
            Ok(())
        } else {
            Err(crate::Error::InvalidData("Incorrect message status"))
        }
    }
}

impl<const STATUS: u16, B: Ump + BufferMut> property::WriteProperty<B> for StatusProperty<STATUS> {
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;

        for packet in buffer.buffer_mut().message_mut().chunks_exact_mut(4) {
            packet[0] &= !0x03FF_0000;
            packet[0] |= (STATUS as u32) << 16;
        }
    }
    fn default() -> Self::Type {
        ()
    }
}

struct ConsistentFormatsProperty;

impl<B: Ump> property::Property<B> for ConsistentFormatsProperty {
    type Type = ();
}

impl<'a, B: Ump> property::ReadProperty<'a, B> for ConsistentFormatsProperty {
    fn read(_buffer: &'a B) -> Self::Type {
        ()
    }

    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::message::helpers::validate_sysex_group_statuses;
        use crate::util::BitOps;

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

impl<B: Ump + BufferMut> property::WriteProperty<B> for ConsistentFormatsProperty {
    fn default() -> Self::Type {
        ()
    }
    fn write(buffer: &mut B, _v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::message::helpers::ump_stream_flex_data::set_format_fields;
        set_format_fields::<UMP_MESSAGE_TYPE>(buffer.buffer_mut().message_mut())
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
}

pub struct TextWriteStrProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> property::Property<B> for TextWriteStrProperty<'a> {
    type Type = &'a str;
}

impl<'a, B: Ump + BufferMut> property::WriteProperty<B> for TextWriteStrProperty<'a> {
    fn write(buffer: &mut B, text: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::message::helpers::ump_stream_flex_data::write_str;
        write_str(buffer.buffer_mut().message_mut(), text);
    }
    fn default() -> Self::Type {
        ""
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<'a, B: Ump + BufferMut> property::ResizeProperty<B> for TextWriteStrProperty<'a> {
    fn resize(buffer: &mut B, value: &Self::Type)
    where
        B: crate::buffer::BufferResize,
    {
        use crate::buffer::UmpPrivateMut;
        use crate::message::helpers::ump_stream_flex_data::{
            clear_payload, required_buffer_size_for_str, set_format_fields,
        };

        let buffer_size = required_buffer_size_for_str(value);
        buffer.resize(buffer_size);
        clear_payload(buffer.buffer_mut().message_mut());

        write_message_header_data(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
        set_format_fields::<UMP_MESSAGE_TYPE>(buffer.buffer_mut().message_mut());
    }

    fn try_resize(buffer: &mut B, value: &Self::Type) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize,
    {
        use crate::buffer::UmpPrivateMut;
        use crate::message::helpers::ump_stream_flex_data::{
            clear_payload, required_buffer_size_for_str, set_format_fields,
        };

        let buffer_size = required_buffer_size_for_str(value);
        buffer.try_resize(buffer_size)?;
        clear_payload(buffer.buffer_mut().message_mut());

        write_message_header_data(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
        set_format_fields::<UMP_MESSAGE_TYPE>(buffer.buffer_mut().message_mut());

        Ok(())
    }
}

fn write_message_header_data(buffer: &mut [u32], size: usize) {
    use crate::numeric_types::u4;
    use crate::util::BitOps;

    let status = u16::from(status_from_buffer(buffer));

    for packet in buffer[..size].chunks_exact_mut(4) {
        packet[0].set_nibble(0, u4::new(UMP_MESSAGE_TYPE));
        packet[0] &= !0x03FF_0000;
        packet[0] |= (status as u32) << 16;
    }

    for packet in buffer[size..].chunks_exact_mut(4) {
        packet[0] = 0x0;
    }
}

fn status_from_buffer(buffer: &[u32]) -> crate::numeric_types::u10 {
    use crate::util::Truncate;
    (buffer[0] >> 16).truncate()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_support::debug;
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn builder() {
//         assert_eq!(
//             debug::Data(
//                 UmpStreamMessage::builder()
//                     .endpoint_name()
//                     .name("RhythmRevelation: Beats Beyond Boundaries🌍🥁🚀")
//                     .build()
//                     .unwrap()
//                     .data()
//             ),
//             debug::Data(&[
//                 0xF403_5268,
//                 0x7974_686D,
//                 0x5265_7665,
//                 0x6C61_7469,
//                 0xF803_6F6E,
//                 0x3A20_4265,
//                 0x6174_7320,
//                 0x4265_796F,
//                 0xF803_6E64,
//                 0x2042_6F75,
//                 0x6E64_6172,
//                 0x6965_73F0,
//                 0xFC03_9F8C,
//                 0x8DF0_9FA5,
//                 0x81F0_9F9A,
//                 0x8000_0000,
//             ]),
//         );
//     }
//
//     #[test]
//     fn into_owned() {
//         let _owned = {
//             let buffer = [
//                 0xF403_5268,
//                 0x7974_686D,
//                 0x5265_7665,
//                 0x6C61_7469,
//                 0xF803_6F6E,
//                 0x3A20_4265,
//                 0x6174_7320,
//                 0x4265_796F,
//                 0xF803_6E64,
//                 0x2042_6F75,
//                 0x6E64_6172,
//                 0x6965_73F0,
//                 0xFC03_9F8C,
//                 0x8DF0_9FA5,
//                 0x81F0_9F9A,
//                 0x8000_0000,
//             ];
//             let message = UmpStreamMessage::from_data(&buffer).unwrap();
//             message.into_owned()
//         };
//     }
// }
