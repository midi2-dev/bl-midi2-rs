use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x20;

#[midi2_attr::generate_message()]
struct StartOfClip {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            StartOfClipMessage::builder().build(),
            Ok(StartOfClipMessage::Owned(StartOfClipOwned([
                0xF020_0000,
                0x0,
                0x0,
                0x0
            ])))
        );
    }

    #[test]
    fn from_data() {
        assert_eq!(
            StartOfClipMessage::from_data(&[0xF020_0000]),
            Ok(StartOfClipMessage::Borrowed(StartOfClipBorrowed(&[
                0xF020_0000
            ])))
        );
    }
}
