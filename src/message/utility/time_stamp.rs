use crate::{
    error::Error,
    message::helpers as message_helpers,
    result::Result,
    util::{debug, BitOps, Truncate},
    *,
};

#[derive(Clone, PartialEq, Eq)]
pub struct TimeStampMessage<'a>(&'a [u32]);

impl<'a> TimeStampMessage<'a> {
    const OP_CODE: u4 = u4::new(0b0010);
    pub fn builder(buffer: &'a mut [u32]) -> TimeStampMessageBuilder<'a> {
        TimeStampMessageBuilder::new(buffer)
    }
    pub fn group(&self) -> u4 {
        self.0[0].nibble(1)
    }
    pub fn time_stamp(&self) -> u20 {
        self.0[0].truncate()
    }
    pub fn from_data(data: &'a [u32]) -> Result<Self> {
        super::validate_packet(data, TimeStampMessage::OP_CODE)?;
        Ok(TimeStampMessage(&data[..1]))
    }
    pub fn data(&self) -> &[u32] {
        self.0
    }
}

debug::message_debug_impl!(TimeStampMessage);

pub struct TimeStampMessageBuilder<'a>(Option<&'a mut [u32]>);

impl<'a> TimeStampMessageBuilder<'a> {
    pub fn group(mut self, g: u4) -> Self {
        if let Some(buffer) = &mut self.0 {
            buffer[0].set_nibble(1, g);
        }
        self
    }
    pub fn time_stamp(mut self, time_stamp: u20) -> Self {
        if let Some(buffer) = &mut self.0 {
            buffer[0] |= u32::from(time_stamp);
        }
        self
    }
    fn new(buffer: &'a mut [u32]) -> Self {
        if !buffer.is_empty() {
            message_helpers::clear_buffer(buffer);
            buffer[0].set_nibble(2, u4::new(2));
            Self(Some(&mut buffer[0..1]))
        } else {
            Self(None)
        }
    }
    pub fn build(self) -> Result<TimeStampMessage<'a>> {
        if let Some(buffer) = self.0 {
            Ok(TimeStampMessage(buffer))
        } else {
            Err(Error::BufferOverflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::random_buffer;

    #[test]
    fn builder() {
        assert_eq!(
            TimeStampMessage::builder(&mut random_buffer::<1>())
                .group(u4::new(0x4))
                .time_stamp(u20::new(0xE_69AE))
                .build(),
            Ok(TimeStampMessage(&[0x042E_69AE])),
        );
    }

    #[test]
    fn builder_default() {
        assert_eq!(
            TimeStampMessage::builder(&mut random_buffer::<1>()).build(),
            Ok(TimeStampMessage(&[0x0020_0000])),
        );
    }

    #[test]
    fn builder_oversized_buffer() {
        assert_eq!(
            TimeStampMessage::builder(&mut random_buffer::<1>()).build(),
            Ok(TimeStampMessage(&[0x0020_0000])),
        );
    }

    #[test]
    fn builder_overflow() {
        assert_eq!(
            TimeStampMessage::builder(&mut []).build(),
            Err(Error::BufferOverflow),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TimeStampMessage::from_data(&[0x0F20_0000]).unwrap().group(),
            u4::new(0xF),
        )
    }

    #[test]
    fn time_stamp() {
        assert_eq!(
            TimeStampMessage::from_data(&[0x0021_2345])
                .unwrap()
                .time_stamp(),
            u20::new(0x12345),
        )
    }
}
