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
mod function_block_name;
mod product_instance_id;
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
            |p| u8::from(p[0].crumb(2)) == COMPLETE_FORMAT,
            |p| u8::from(p[0].crumb(2)) == START_FORMAT,
            |p| u8::from(p[0].crumb(2)) == CONTINUE_FORMAT,
            |p| u8::from(p[0].crumb(2)) == END_FORMAT,
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
        set_format_fields(buffer.buffer_mut().message_mut())
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
}

pub struct TextWriteStrProperty<'a, const OFFSET: usize>(core::marker::PhantomData<&'a u8>);

impl<'a, const OFFSET: usize, B: Ump> property::Property<B> for TextWriteStrProperty<'a, OFFSET> {
    type Type = &'a str;
}

impl<'a, const OFFSET: usize, B: Ump + BufferMut> property::WriteProperty<B>
    for TextWriteStrProperty<'a, OFFSET>
{
    fn write(buffer: &mut B, text: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::util::BitOps;

        let mut packet_index = 0;
        let mut byte_index = 0;

        for b in text.as_bytes() {
            buffer.buffer_mut().message_mut()[packet_index * 4 + (byte_index + 2 + OFFSET) / 4]
                .set_octet((byte_index + 2 + OFFSET) % 4, *b);

            if byte_index == 13 - OFFSET {
                // end of the packet
                packet_index += 1;
                byte_index = 0;
            } else {
                byte_index += 1;
            }
        }
    }
    fn default() -> Self::Type {
        ""
    }
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
}

impl<'a, const OFFSET: usize, B: Ump + BufferMut> property::ResizeProperty<B>
    for TextWriteStrProperty<'a, OFFSET>
{
    fn resize(buffer: &mut B, value: &Self::Type)
    where
        B: crate::buffer::BufferResize,
    {
        use crate::buffer::UmpPrivateMut;

        let buffer_size = required_buffer_size_for_str::<OFFSET>(value);
        buffer.resize(buffer_size);
        clear_payload::<OFFSET>(buffer.buffer_mut().message_mut());

        write_message_header_data(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
        set_format_fields(buffer.buffer_mut().message_mut());
    }

    fn try_resize(buffer: &mut B, value: &Self::Type) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize,
    {
        use crate::buffer::UmpPrivateMut;

        let buffer_size = required_buffer_size_for_str::<OFFSET>(value);
        buffer.try_resize(buffer_size)?;
        clear_payload::<OFFSET>(buffer.buffer_mut().message_mut());

        write_message_header_data(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
        set_format_fields(buffer.buffer_mut().message_mut());

        Ok(())
    }
}

pub struct TextBytesIterator<'a> {
    buffer: &'a [u32],
    packet_index: usize,
    byte_index: usize,
    offset: usize,
}

impl<'a> core::iter::Iterator for TextBytesIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished() {
            return None;
        }
        let ret = Some(self.value());
        self.advance();
        while !self.finished() && self.value() == 0 {
            self.advance();
        }
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        ((self.buffer.len() - 1) * 14, Some(self.buffer.len() * 14))
    }
}

impl<'a> core::iter::FusedIterator for TextBytesIterator<'a> {}

impl<'a> TextBytesIterator<'a> {
    fn finished(&self) -> bool {
        self.buffer.len() / 4 <= self.packet_index
    }
    fn advance(&mut self) {
        self.byte_index += 1;
        if self.byte_index == 14 - self.offset {
            // end of message
            self.packet_index += 1;
            self.byte_index = 0;
        }
    }
    fn value(&mut self) -> u8 {
        use crate::util::BitOps;
        let buffer_index = self.packet_index * 4 + (self.byte_index + 2 + self.offset) / 4;
        let byte_index = (self.byte_index + 2 + self.offset) % 4;
        self.buffer[buffer_index].octet(byte_index)
    }
}

pub struct TextReadBytesProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> property::Property<B> for TextReadBytesProperty<'a> {
    type Type = TextBytesIterator<'a>;
}

impl<'a, B: 'a + Ump> property::ReadProperty<'a, B> for TextReadBytesProperty<'a> {
    fn read(buffer: &'a B) -> <Self as property::Property<B>>::Type {
        use crate::buffer::UmpPrivate;
        TextBytesIterator {
            buffer: buffer.buffer().message(),
            packet_index: 0,
            byte_index: 0,
            offset: 0,
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "std")]
pub struct TextReadStringProperty;

#[cfg(feature = "std")]
impl<B: Ump> property::Property<B> for TextReadStringProperty {
    type Type = std::string::String;
}

#[cfg(feature = "std")]
impl<'a, B: Ump> property::ReadProperty<'a, B> for TextReadStringProperty {
    fn read(buffer: &'a B) -> Self::Type {
        let bytes = TextReadBytesProperty::read(buffer).collect();
        std::string::String::from_utf8(bytes).unwrap()
    }
    fn validate(buffer: &B) -> crate::result::Result<()> {
        let bytes = TextReadBytesProperty::read(buffer).collect();
        std::string::String::from_utf8(bytes).map_err(|_| {
            crate::Error::InvalidData("Payload bytes do not represent a valid utf string")
        })?;
        Ok(())
    }
}

fn set_format_fields(buffer: &mut [u32]) {
    use crate::numeric_types::u2;
    use crate::util::BitOps;

    let mut packets = buffer
        .chunks_exact_mut(4)
        .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
        .peekable();

    let Some(first) = packets.next() else {
        panic!("Can't be called with empty buffer");
    };

    if packets.peek().is_some() {
        first[0].set_crumb(2, u2::new(START_FORMAT));
    } else {
        first[0].set_crumb(2, u2::new(COMPLETE_FORMAT));
    }

    while let Some(packet) = packets.next() {
        if packets.peek().is_some() {
            packet[0].set_crumb(2, u2::new(CONTINUE_FORMAT));
        } else {
            packet[0].set_crumb(2, u2::new(END_FORMAT));
        }
    }
}

fn clear_payload<const OFFSET: usize>(buffer: &mut [u32]) {
    debug_assert!(OFFSET < 2);
    for packet in buffer.chunks_exact_mut(4) {
        use crate::util::BitOps;
        if OFFSET < 1 {
            packet[0].set_octet(2, 0);
        }
        if OFFSET < 2 {
            packet[0].set_octet(3, 0);
        }
        packet[1] = 0x0;
        packet[2] = 0x0;
        packet[3] = 0x0;
    }
}

fn required_buffer_size_for_str<const OFFSET: usize>(s: &str) -> usize {
    let str_size = s.as_bytes().len();
    let packet_capacity = 14 - OFFSET;
    let ret = if str_size % packet_capacity == 0 {
        if str_size == 0 {
            4
        } else {
            str_size * 4 / packet_capacity
        }
    } else {
        4 * (str_size / packet_capacity + 1)
    };
    ret + crate::buffer::OFFSET_FOR_JITTER_REDUCTION
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

fn message_size<B: crate::buffer::Ump>(buffer: &B) -> usize {
    use crate::buffer::UmpPrivate;
    use crate::util::BitOps;

    let jr_offset = buffer.buffer().jitter_reduction().len();
    buffer
        .buffer()
        .message()
        .chunks_exact(4)
        .position(|p| {
            let format: u8 = p[0].crumb(2).into();
            format == COMPLETE_FORMAT || format == END_FORMAT
        })
        .expect("Message is in an invalid state. Couldn't find end packet.")
        * 4
        + 4
        + jr_offset
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
