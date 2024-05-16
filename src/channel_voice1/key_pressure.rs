use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1010;

/// MIDI 1.0 Channel Voice Key Pressure Message
///
/// See the [module docs](crate::channel_voice1) for more info.
#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct KeyPressure {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::ChannelProperty)]
    channel: crate::ux::u4,
    #[property(common_properties::GroupProperty)]
    group: crate::ux::u4,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x7F, 0x0>,
        schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>,
    >)]
    note: crate::ux::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    pressure: crate::ux::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{Channeled, Grouped},
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = KeyPressure::<[u32; 4]>::new();
        message.set_group(u4::new(0xA));
        message.set_channel(u4::new(0x3));
        message.set_note(u7::new(0x7F));
        message.set_pressure(u7::new(0x5C));
        assert_eq!(message, KeyPressure([0x2AA3_7F5C, 0x0, 0x0, 0x0]));
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
