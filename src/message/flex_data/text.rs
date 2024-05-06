use crate::{
    buffer::{BufferMut, Ump},
    message::{common_properties, flex_data},
    util::property::{Property, ReadProperty, ResizeProperty, WriteProperty},
};

pub struct TextWriteStrProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> Property<B> for TextWriteStrProperty<'a> {
    type Type = &'a str;
}

impl<'a, B: Ump + BufferMut> WriteProperty<B> for TextWriteStrProperty<'a> {
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

impl<'a, B: Ump + BufferMut> ResizeProperty<B> for TextWriteStrProperty<'a> {
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
        set_format_fields::<{ flex_data::UMP_MESSAGE_TYPE }>(buffer.buffer_mut().message_mut());
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
        set_format_fields::<{ flex_data::UMP_MESSAGE_TYPE }>(buffer.buffer_mut().message_mut());
        Ok(())
    }
}

fn write_message_header_data(buffer: &mut [u32], size: usize) {
    use crate::numeric_types::u4;
    use crate::util::BitOps;

    let group = <common_properties::GroupProperty as ReadProperty<&[u32]>>::read(&&buffer[..5]);
    let channel = <flex_data::OptionalChannelProperty as ReadProperty<&[u32]>>::read(&&buffer[..5]);
    let bank = flex_data::bank_from_buffer(buffer);
    let status = flex_data::status_from_buffer(buffer);

    for packet in buffer[..size].chunks_exact_mut(4) {
        packet[0].set_nibble(0, u4::new(flex_data::UMP_MESSAGE_TYPE));
        packet[0].set_nibble(1, group);
        flex_data::optional_channel_to_slice(packet, channel);
        flex_data::status_to_buffer(packet, status);
        flex_data::bank_to_buffer(packet, bank);
    }

    for packet in buffer[size..].chunks_exact_mut(4) {
        packet[0] = 0x0;
    }
}

pub struct TextBytesIterator<'a> {
    buffer: &'a [u32],
    packet_index: usize,
    byte_index: usize,
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
        ((self.buffer.len() - 1) * 12, Some(self.buffer.len() * 12))
    }
}

impl<'a> core::iter::FusedIterator for TextBytesIterator<'a> {}

impl<'a> TextBytesIterator<'a> {
    fn finished(&self) -> bool {
        self.buffer.len() / 4 <= self.packet_index
    }
    fn advance(&mut self) {
        self.byte_index += 1;
        if self.byte_index == 12 {
            // end of message
            self.packet_index += 1;
            self.byte_index = 0;
        }
    }
    fn value(&mut self) -> u8 {
        use crate::util::BitOps;
        let buffer_index = self.packet_index * 4 + 1 + self.byte_index / 4;
        let byte_index = self.byte_index % 4;
        self.buffer[buffer_index].octet(byte_index)
    }
}

pub struct TextReadBytesProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> Property<B> for TextReadBytesProperty<'a> {
    type Type = TextBytesIterator<'a>;
}

impl<'a, B: 'a + Ump> ReadProperty<'a, B> for TextReadBytesProperty<'a> {
    fn read(buffer: &'a B) -> <Self as Property<B>>::Type {
        use crate::buffer::UmpPrivate;
        TextBytesIterator {
            buffer: buffer.buffer().message(),
            packet_index: 0,
            byte_index: 0,
        }
    }
    fn validate(_buffer: &B) -> crate::result::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "std")]
pub struct TextReadStringProperty;

#[cfg(feature = "std")]
impl<B: Ump> Property<B> for TextReadStringProperty {
    type Type = std::string::String;
}

#[cfg(feature = "std")]
impl<'a, B: Ump> ReadProperty<'a, B> for TextReadStringProperty {
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
