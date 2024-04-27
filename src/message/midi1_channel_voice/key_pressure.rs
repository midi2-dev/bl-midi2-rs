use crate::{
    message::{common_properties, midi1_channel_voice::UMP_MESSAGE_TYPE},
    util::schema,
};

const STATUS: u8 = 0b1010;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(3))]
struct KeyPressure {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::numeric_types::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    note: crate::numeric_types::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::numeric_types::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    pressure: crate::numeric_types::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        numeric_types::*,
        traits::{Channeled, Grouped},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = KeyPressure::new_arr();
        message.set_group(u4::new(0xA));
        message.set_channel(u4::new(0x3));
        message.set_note(u7::new(0x7F));
        message.set_pressure(u7::new(0x5C));
        assert_eq!(message, KeyPressure([0x2AA3_7F5C]));
    }

    #[test]
    fn group() {
        assert_eq!(
            KeyPressure::try_from(&[0x2AA3_7F5C_u32][..])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            KeyPressure::try_from(&[0x2AA3_7F5C_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x3),
        );
    }

    #[test]
    fn note() {
        assert_eq!(
            KeyPressure::try_from(&[0x2AA3_7F5C_u32][..])
                .unwrap()
                .note(),
            u7::new(0x7F),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            KeyPressure::try_from(&[0x2AA3_7F5C_u32][..])
                .unwrap()
                .pressure(),
            u7::new(0x5C),
        );
    }
}
