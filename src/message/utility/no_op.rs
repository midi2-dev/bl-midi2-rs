use crate::util::{self, BitOps};

impl<'a> util::message::MessagePrivate<'a, 4> for NoOpMessage<'a> {
    type Owned<'b: 'a> = NoOpMessage<'b>;
    fn new(message_impl: util::message::MessageImpl<'a, 4>) -> Self {
        NoOpMessage(message_impl)
    }
    fn message_impl(&self) -> &util::message::MessageImpl<'a, 4> {
        &self.0
    }
    fn message_impl_mut(&mut self) -> &mut util::message::MessageImpl<'a, 4> {
        &mut self.0
    }
    fn default_data() -> [u32; 4] {
        [0; 4]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoOpMessage<'a>(util::message::MessageImpl<'a, 4>);

impl<'a> util::message::Message<'a, 4> for NoOpMessage<'a> {
    fn validate(data: &[u32]) -> Result<(), crate::error::InvalidData> {
        super::validate_packet_2(data, NoOpMessage::OP_CODE)
    }
}

impl<'a> NoOpMessage<'a> {
    const OP_CODE: ux::u4 = ux::u4::new(0x0);
    pub fn group(&self) -> ux::u4 {
        self.0[0].nibble(1)
    }
    pub fn set_group(&mut self, g: ux::u4) -> &mut Self {
        self.0[0].set_nibble(1, g);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error, util::message::Message};

    #[test]
    fn validate() {
        assert_eq!(NoOpMessage::validate(&[0x0700_0000]), Ok(()));
    }

    #[test]
    fn validate_fail() {
        assert_eq!(
            NoOpMessage::validate(&[0x1700_0000]),
            Err(error::InvalidData {})
        );
    }

    #[test]
    fn set_group() {
        assert_eq!(
            NoOpMessage::default().set_group(ux::u4::new(0x2)),
            &mut NoOpMessage::try_new_owned(&[0x0200_0000, 0x0, 0x0, 0x0]).unwrap(),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            NoOpMessage::try_new_borrowed(&[0x0200_0000])
                .unwrap()
                .group(),
            ux::u4::new(0x2),
        );
    }
}
