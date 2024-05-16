use crate::{
    channel_voice2::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
    ux::{u4, u7},
};

pub(crate) const STATUS: u8 = 0b1011;

/// MIDI 2.0 Channel Voice Control Change Message
///
/// See the [module docs](crate::channel_voice2) for more info.
#[midi2_proc::generate_message(Via(crate::channel_voice2::ChannelVoice2), FixedSize, MinSizeUmp(2))]
struct ControlChange {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(common_properties::UmpSchemaProperty<u4, schema::Ump<0x000F_0000, 0x0, 0x0, 0x0>>)]
    channel: u4,
    #[property(common_properties::GroupProperty)]
    group: u4,
    #[property(common_properties::UmpSchemaProperty<u7, schema::Ump<0x0000_7F00, 0x0, 0x0, 0x0>>)]
    control: u7,
    #[property(common_properties::UmpSchemaProperty<u32, schema::Ump<0x0000_0000, 0xFFFF_FFFF, 0x0, 0x0>>)]
    control_change_data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        use crate::traits::{Channeled, Grouped};
        let mut message = ControlChange::<[u32; 4]>::new();
        message.set_group(u4::new(0x3));
        message.set_channel(u4::new(0x9));
        message.set_control(u7::new(0x30));
        message.set_control_change_data(0x2468_1012);

        assert_eq!(message, ControlChange([0x43B9_3000, 0x2468_1012, 0x0, 0x0]));
    }

    #[test]
    fn control() {
        assert_eq!(
            ControlChange::try_from(&[0x43B9_3000, 0x2468_1012][..])
                .unwrap()
                .control(),
            u7::new(0x30),
        );
    }

    #[test]
    fn control_change_data() {
        assert_eq!(
            ControlChange::try_from(&[0x43B9_3000, 0x2468_1012][..])
                .unwrap()
                .control_change_data(),
            0x2468_1012,
        );
    }
}
