use crate::{detail::common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE};

pub(crate) const STATUS: u16 = 0x4;

#[midi2_proc::generate_message(Via(ump_stream::UmpStream), MinSizeUmp(4))]
struct ProductInstanceId {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(ump_stream::TextWriteStrProperty<0>)]
    #[writeonly]
    #[resize]
    id: &str,
    #[property(ump_stream::TextReadBytesProperty)]
    #[readonly]
    id_bytes: ump_stream::TextBytesIterator,
    #[property(ump_stream::TextReadStringProperty)]
    #[readonly]
    #[std]
    id: std::string::String,
}

impl<B: crate::buffer::Ump> crate::traits::Size<B> for ProductInstanceId<B> {
    fn size(&self) -> usize {
        ump_stream::message_size(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn set_id() {
        let mut message = ProductInstanceId::<std::vec::Vec<u32>>::new();
        message.set_id("PianoPulse");
        assert_eq!(
            message,
            ProductInstanceId(std::vec![
                0xF004_5069,
                0x616E_6F50,
                0x756C_7365,
                0x0000_0000
            ]),
        )
    }

    #[test]
    #[cfg(feature = "std")]
    fn id() {
        assert_eq!(
            ProductInstanceId::try_from(&[0xF004_5069, 0x616E_6F50, 0x756C_7365, 0x0000_0000,][..])
                .unwrap()
                .id(),
            "PianoPulse",
        )
    }
}
