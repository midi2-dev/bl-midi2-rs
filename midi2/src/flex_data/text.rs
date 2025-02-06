use crate::{
    buffer::{BufferMut, Ump},
    detail::property::{Property, ReadProperty, ResizeProperty, WriteProperty},
    flex_data,
};

pub struct TextWriteStrProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> Property<B> for TextWriteStrProperty<'a> {
    type Type = &'a str;
}

impl<B: Ump + BufferMut> WriteProperty<B> for TextWriteStrProperty<'_> {
    fn write(buffer: &mut B, text: Self::Type) {
        use crate::detail::BitOps;

        let mut packet_index = 0;
        let mut byte_index = 0;

        for b in text.as_bytes() {
            buffer.buffer_mut()[packet_index * 4 + 1 + byte_index / 4]
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
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
}

impl<B: Ump + BufferMut> ResizeProperty<B> for TextWriteStrProperty<'_> {
    fn resize(buffer: &mut B, value: &Self::Type)
    where
        B: crate::buffer::BufferResize,
    {
        let buffer_size = ump_buffer_size_for_str(value);
        buffer.resize(buffer_size);
        flex_data::clear_payload(buffer.buffer_mut());
        grow_buffer(buffer.buffer_mut(), buffer_size);
    }

    fn try_resize(buffer: &mut B, value: &Self::Type) -> Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferTryResize,
    {
        let buffer_size = ump_buffer_size_for_str(value);
        buffer.try_resize(buffer_size)?;
        flex_data::clear_payload(buffer.buffer_mut());
        grow_buffer(buffer.buffer_mut(), buffer_size);
        Ok(())
    }
}

fn ump_buffer_size_for_str(s: &str) -> usize {
    let str_size = s.len();
    if str_size % 12 == 0 {
        if str_size == 0 {
            4
        } else {
            str_size * 4 / 12
        }
    } else {
        4 * (str_size / 12 + 1)
    }
}

fn grow_buffer(mut buffer: &mut [u32], size: usize) {
    use crate::detail::BitOps;
    use crate::ux::{u2, u4};

    debug_assert!(size % 4 == 0);
    debug_assert!(size != 0);

    let group = buffer[0].nibble(1);
    let channel = flex_data::optional_channel_from_slice(buffer);
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

impl core::iter::Iterator for TextBytesIterator<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished() && self.value() == 0 {
            self.advance();
        }
        if self.finished() {
            return None;
        }
        let ret = Some(self.value());
        self.advance();
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        ((self.buffer.len() - 1) * 12, Some(self.buffer.len() * 12))
    }
}

impl core::iter::FusedIterator for TextBytesIterator<'_> {}

impl TextBytesIterator<'_> {
    fn finished(&self) -> bool {
        self.packet_index == self.buffer.len() / 4 - 1 && self.byte_index == 11
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
        use crate::detail::BitOps;
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
        TextBytesIterator {
            buffer: buffer.buffer(),
            packet_index: 0,
            byte_index: 0,
        }
    }
    fn validate(_buffer: &B) -> Result<(), crate::error::InvalidData> {
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
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        let bytes = TextReadBytesProperty::read(buffer).collect();
        std::string::String::from_utf8(bytes).map_err(|_| {
            crate::error::InvalidData("Payload bytes do not represent a valid utf string")
        })?;
        Ok(())
    }
}
