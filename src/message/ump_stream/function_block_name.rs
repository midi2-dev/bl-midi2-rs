use crate::{
    buffer::{BufferMut, Ump},
    message::{common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE},
    util::property,
};

pub(crate) const STATUS: u16 = 0x12;

#[midi2_proc::generate_message(MinSizeUmp(4))]
struct FunctionBlockName {
    #[property(crate::message::utility::JitterReductionProperty)]
    jitter_reduction: Option<crate::message::utility::JitterReduction>,
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(FunctionBlockProperty)]
    function_block: u8,
    #[property(ump_stream::TextWriteStrProperty<1>)]
    #[writeonly]
    #[resize]
    name: &str,
    #[property(TextReadBytesProperty)]
    #[readonly]
    name_bytes: ump_stream::TextBytesIterator,
    #[property(TextReadStringProperty)]
    #[readonly]
    #[std]
    name: std::string::String,
}

impl<B: Ump> crate::traits::Size<B> for FunctionBlockName<B> {
    fn size(&self) -> usize {
        ump_stream::message_size(&self.0)
    }
}

struct FunctionBlockProperty;

impl<B: Ump> property::Property<B> for FunctionBlockProperty {
    type Type = u8;
}

impl<'a, B: Ump> property::ReadProperty<'a, B> for FunctionBlockProperty {
    fn validate(buffer: &B) -> crate::result::Result<()> {
        use crate::buffer::UmpPrivate;
        use crate::util::BitOps;

        let function_block = buffer.buffer().message()[0].octet(2);
        if !buffer
            .buffer()
            .message()
            .chunks_exact(4)
            .all(|packet| packet[0].octet(2) == function_block)
        {
            Err(crate::error::Error::InvalidData(
                "Inconsistent function block fields",
            ))
        } else {
            Ok(())
        }
    }
    fn read(buffer: &'a B) -> Self::Type {
        use crate::buffer::UmpPrivate;
        use crate::util::BitOps;
        buffer.buffer().message()[0].octet(2)
    }
}

impl<B: Ump + BufferMut> property::WriteProperty<B> for FunctionBlockProperty {
    fn validate(_v: &Self::Type) -> crate::result::Result<()> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
    fn write(buffer: &mut B, v: Self::Type) {
        use crate::buffer::UmpPrivateMut;
        use crate::util::BitOps;
        for packet in buffer.buffer_mut().message_mut().chunks_exact_mut(4) {
            packet[0].set_octet(2, v);
        }
    }
}

pub struct TextReadBytesProperty<'a>(core::marker::PhantomData<&'a u8>);

impl<'a, B: Ump> property::Property<B> for TextReadBytesProperty<'a> {
    type Type = ump_stream::TextBytesIterator<'a>;
}

impl<'a, B: 'a + Ump> property::ReadProperty<'a, B> for TextReadBytesProperty<'a> {
    fn read(buffer: &'a B) -> <Self as property::Property<B>>::Type {
        use crate::buffer::UmpPrivate;
        ump_stream::TextBytesIterator {
            buffer: buffer.buffer().message(),
            packet_index: 0,
            byte_index: 0,
            offset: 1,
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_name() {
        let mut message = FunctionBlockName::new();
        message.set_name("SynthWave🌊²");
        message.set_function_block(0x09);
        assert_eq!(
            message,
            FunctionBlockName(std::vec![
                0x0,
                0xF412_0953,
                0x796E_7468,
                0x5761_7665,
                0xF09F_8C8A,
                0xFC12_09C2,
                0xB200_0000,
                0x0000_0000,
                0x0000_0000,
            ])
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn name() {
        assert_eq!(
            FunctionBlockName::try_from(
                &[
                    0xF412_0953,
                    0x796E_7468,
                    0x5761_7665,
                    0xF09F_8C8A,
                    0xFC12_09C2,
                    0xB200_0000,
                    0x0000_0000,
                    0x0000_0000,
                ][..]
            )
            .unwrap()
            .name(),
            "SynthWave🌊²",
        );
    }

    #[test]
    fn function_block() {
        assert_eq!(
            FunctionBlockName::try_from(
                &[
                    0xF412_0953,
                    0x796E_7468,
                    0x5761_7665,
                    0xF09F_8C8A,
                    0xFC12_09C2,
                    0xB200_0000,
                    0x0000_0000,
                    0x0000_0000,
                ][..]
            )
            .unwrap()
            .function_block(),
            0x9,
        );
    }
}
