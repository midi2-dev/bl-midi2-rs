use crate::{
    error::Error,
    message::helpers as message_helpers,
    result::Result,
    util::{debug, BitOps},
    *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct NoOpMessage<'a>(&'a [u32]);

impl<'a> NoOpMessage<'a> {
    const OP_CODE: u4 = u4::new(0x0);
}

impl<'a> Message<'a> for NoOpMessage<'a> {
    type Builder = NoOpBuilder<'a>;
    fn from_data_unchecked(data: &'a [u32]) -> Self {
        NoOpMessage(&data[..1])
    }
    fn data(&self) -> &'a [u32] {
        self.0
    }
    fn validate_data(buffer: &'a [u32]) -> Result<()> {
        super::validate_packet(buffer, NoOpMessage::OP_CODE)
    }
}

impl<'a> GroupedMessage<'a> for NoOpMessage<'a> {
    fn group(&self) -> u4 {
        self.0[0].nibble(1)
    }
}

debug::message_debug_impl!(NoOpMessage);

pub struct NoOpBuilder<'a>(Option<&'a mut [u32]>);

impl<'a> Builder<'a> for NoOpBuilder<'a> {
    type Message = NoOpMessage<'a>;
    fn new(buffer: &'a mut [u32]) -> Self {
        if !buffer.is_empty() {
            message_helpers::clear_buffer(buffer);
            Self(Some(&mut buffer[..1]))
        } else {
            Self(None)
        }
    }
    fn build(self) -> Result<NoOpMessage<'a>> {
        if let Some(buffer) = self.0 {
            Ok(NoOpMessage(buffer))
        } else {
            Err(Error::BufferOverflow)
        }
    }
}

impl<'a> GroupedBuilder<'a> for NoOpBuilder<'a> {
    fn group(mut self, g: u4) -> Self {
        if let Some(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            NoOpMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0xB))
                .build(),
            Ok(NoOpMessage(&[0x0B00_0000])),
        )
    }

    #[test]
    fn group() {
        assert_eq!(
            NoOpMessage::from_data(&[0x0900_0000]).unwrap().group(),
            u4::new(0x9),
        );
    }
}
