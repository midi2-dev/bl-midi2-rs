use crate::{detail::common_properties, ump_stream, ump_stream::UMP_MESSAGE_TYPE};

pub(crate) const STATUS: u16 = 0x20;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1))]
struct StartOfClip {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            StartOfClip::new_arr(),
            StartOfClip([0xF020_0000, 0x0, 0x0, 0x0]),
        );
    }

    #[test]
    fn from_data() {
        assert_eq!(
            StartOfClip::try_from(&[0xF020_0000][..]),
            Ok(StartOfClip(&[0xF020_0000][..])),
        );
    }
}
