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
        use crate::util::BitOps;

        let mut packet_index = 0;
        let mut byte_index = 0;

        for b in text.as_bytes() {
            buffer.buffer_mut().message_mut()[packet_index * 4 + 1 + byte_index / 4]
                .set_octet(byte_index % 4, *b);

            if byte_index == 11 {
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

impl<'a, B: Ump + BufferMut> ResizeProperty<B> for TextWriteStrProperty<'a> {
    fn resize(buffer: &mut B, value: &Self::Type)
    where
        B: crate::buffer::BufferResize,
    {
        use crate::buffer::UmpPrivateMut;

        let buffer_size = ump_buffer_size_for_str(value);
        buffer.resize(buffer_size);
        flex_data::clear_payload(buffer.buffer_mut().message_mut());
        grow_buffer(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
    }

    fn try_resize(buffer: &mut B, value: &Self::Type) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize,
    {
        use crate::buffer::UmpPrivateMut;

        let buffer_size = ump_buffer_size_for_str(value);
        buffer.try_resize(buffer_size)?;
        flex_data::clear_payload(buffer.buffer_mut().message_mut());
        grow_buffer(
            buffer.buffer_mut().message_mut(),
            buffer_size - crate::buffer::OFFSET_FOR_JITTER_REDUCTION,
        );
        Ok(())
    }
}

fn ump_buffer_size_for_str(s: &str) -> usize {
    let str_size = s.as_bytes().len();
    let ret = if str_size % 12 == 0 {
        if str_size == 0 {
            4
        } else {
            str_size * 4 / 12
        }
    } else {
        4 * (str_size / 12 + 1)
    };
    ret + crate::buffer::OFFSET_FOR_JITTER_REDUCTION
}

fn grow_buffer(mut buffer: &mut [u32], size: usize) {
    use crate::numeric_types::{u2, u4};
    use crate::util::BitOps;

    debug_assert!(size % 4 == 0);
    debug_assert!(size != 0);

    let group = <common_properties::GroupProperty as ReadProperty<&[u32]>>::read(&&buffer[..5]);
    let channel = <flex_data::OptionalChannelProperty as ReadProperty<&[u32]>>::read(&&buffer[..5]);
    let bank = flex_data::bank_from_buffer(buffer);
    let status = flex_data::status_from_buffer(buffer);
    let number_of_packets = size / 4;

    for (i, packet) in buffer.buffer_mut().chunks_exact_mut(4).enumerate() {
        if i == number_of_packets {
            break;
        }

        // set the format field
        if i == 0 {
            if number_of_packets == 1 {
                packet[0].set_crumb(4, u2::new(flex_data::COMPLETE_FORMAT));
            } else {
                packet[0].set_crumb(4, u2::new(flex_data::START_FORMAT));
            }
        } else if i == number_of_packets - 1 {
            packet[0].set_crumb(4, u2::new(flex_data::END_FORMAT));
        } else {
            packet[0].set_crumb(4, u2::new(flex_data::CONTINUE_FORMAT));
        }

        // set the header data
        packet[0].set_nibble(0, u4::new(flex_data::UMP_MESSAGE_TYPE));
        packet[0].set_nibble(1, group);
        flex_data::optional_channel_to_slice(packet, channel);
        flex_data::status_to_buffer(packet, status);
        flex_data::bank_to_buffer(packet, bank);
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
