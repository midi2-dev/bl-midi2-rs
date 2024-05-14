use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1011;

#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(3)
)]
struct ControlChange {
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
    control: crate::ux::u7,
    #[property(common_properties::HybridSchemaProperty<
        crate::ux::u7,
        schema::Bytes<0x00, 0x0, 0x7F>,
        schema::Ump<0x0000_007F, 0x0, 0x0, 0x0>,
    >)]
    control_data: crate::ux::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ux::*, Channeled, Grouped, Packets};
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = ControlChange::<[u32; 4]>::new();
        message.set_group(u4::new(0xA));
        message.set_channel(u4::new(0x7));
        message.set_control(u7::new(0x36));
        message.set_control_data(u7::new(0x37));
        assert_eq!(message, ControlChange([0x2AB7_3637, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = ControlChange::<[u8; 3]>::new();
        message.set_channel(u4::new(0x7));
        message.set_control(u7::new(0x36));
        message.set_control_data(u7::new(0x37));
        assert_eq!(message, ControlChange([0xB7, 0x36, 0x37]));
    }

    #[test]
    fn group() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .group(),
            u4::new(0xA),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .channel(),
            u4::new(0x7),
        );
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .control(),
            u7::new(0x36),
        );
    }

    #[test]
    fn control_data() {
        assert_eq!(
            ControlChange::try_from(&[0x2AB7_3637_u32][..])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }

    #[test]
    fn control_data_bytes() {
        assert_eq!(
            ControlChange::try_from(&[0xB7_u8, 0x36_u8, 0x37_u8][..])
                .unwrap()
                .control_data(),
            u7::new(0x37),
        );
    }

    #[test]
    fn packets() {
        let buffer = [0x2AB7_3637_u32];
        let message = ControlChange::try_from(&buffer[..]).unwrap();
        let mut packets = message.packets();
        assert_eq!(packets.next(), Some(&[0x2AB7_3637_u32][..]));
        assert_eq!(packets.next(), None);
    }
}
