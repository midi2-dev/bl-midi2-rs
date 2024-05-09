use crate::{detail::common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE};

pub(crate) const STATUS: u16 = 0x3;

#[midi2_proc::generate_message(MinSizeUmp(4))]
struct EndpointName {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(ump_stream::TextWriteStrProperty<0>)]
    #[writeonly]
    #[resize]
    name: &str,
    #[property(ump_stream::TextReadBytesProperty)]
    #[readonly]
    name_bytes: ump_stream::TextBytesIterator,
    #[property(ump_stream::TextReadStringProperty)]
    #[readonly]
    #[std]
    name: std::string::String,
}

impl<B: crate::buffer::Ump> crate::traits::Size<B> for EndpointName<B> {
    fn size(&self) -> usize {
        ump_stream::message_size(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn data() {
        use crate::traits::Data;
        assert_eq!(
            EndpointName::try_from(
                &[
                    0xF403_4769,
                    0x6D6D_6520,
                    0x736F_6D65,
                    0x2073_6967,
                    0xFC03_6E61,
                    0x6C20_F09F,
                    0x948A_20F0,
                    0x9F99_8C00,
                ][..]
            )
            .unwrap()
            .data(),
            &[
                0xF403_4769,
                0x6D6D_6520,
                0x736F_6D65,
                0x2073_6967,
                0xFC03_6E61,
                0x6C20_F09F,
                0x948A_20F0,
                0x9F99_8C00,
            ]
        );
    }

    #[test]
    fn set_name_and_clear_name() {
        let mut message = EndpointName::new();
        message.set_name("Gimme some signal 🔊 🙌");
        message.set_name("");
        assert_eq!(
            message,
            EndpointName(std::vec![
                0xF003_0000,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn get_name() {
        let buffer = [
            0xF403_4769,
            0x6D6D_6520,
            0x736F_6D65,
            0x2073_6967,
            0xFC03_6E61,
            0x6C20_F09F,
            0x948A_20F0,
            0x9F99_8C00,
        ];
        let message = EndpointName::try_from(&buffer[..]).unwrap();
        assert_eq!(message.name(), "Gimme some signal 🔊 🙌");
    }
}
