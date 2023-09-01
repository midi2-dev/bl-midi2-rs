use crate::{
    *,
    error::Error,
    result::Result,
    util::{debug, BitOps},
};

#[derive(Clone, PartialEq, Eq)]
pub struct NoOpMessage<'a>(&'a [u32]);

impl<'a> NoOpMessage<'a> {
    const OP_CODE: u4 = u4::new(0x0);
    pub fn builder(buffer: &'a mut [u32]) -> NoOpMessageBuilder<'a> {
        NoOpMessageBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0[0].nibble(1)
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        super::validate_packet(data, NoOpMessage::OP_CODE)?;
        Ok(NoOpMessage(&data[..1]))
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

debug::message_debug_impl!(NoOpMessage);

pub struct NoOpMessageBuilder<'a>(Option<&'a mut [u32]>);

impl<'a> NoOpMessageBuilder<'a> {
    pub fn group(&mut self, g: u4) -> &mut Self {
        if let Some(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if !buffer.is_empty() {
            let buffer = &mut buffer[..1];
            for v in buffer.iter_mut() {
                *v = 0;
            }
            Self(Some(buffer))
        } else {
            Self(None)
        }
    }
    pub fn build(&'a self) -> Result<NoOpMessage<'a>> {
        if let Some(buffer) = &self.0 {
            Ok(NoOpMessage(buffer))
        } else {
            Err(Error::BufferOverflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder() {
        assert_eq!(
            NoOpMessage::builder(&mut [0x0])
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
