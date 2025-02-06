use crate::{
    detail::{common_properties, schema},
    ump_stream,
    ump_stream::UMP_MESSAGE_TYPE,
};

pub(crate) const STATUS: u16 = 0x6;

#[midi2_proc::generate_message(Via(ump_stream::UmpStream), FixedSize, MinSizeUmp(1))]
struct StreamConfigurationNotification {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ump_stream::StatusProperty<STATUS>)]
    status: (),
    #[property(ump_stream::ConsistentFormatsProperty)]
    consistent_formats: (),
    #[property(common_properties::UmpSchemaProperty<u8, schema::Ump<0x0000_FF00, 0x0, 0x0, 0x0>>)]
    protocol: u8,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0, 0x0>>)]
    receive_jr_timestamps: bool,
    #[property(common_properties::UmpSchemaProperty<bool, schema::Ump<0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0, 0x0>>)]
    send_jr_timestamps: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        let mut message = StreamConfigurationNotification::<[u32; 4]>::new();
        message.set_protocol(0x2);
        message.set_receive_jr_timestamps(true);
        message.set_send_jr_timestamps(true);
        assert_eq!(
            message,
            StreamConfigurationNotification([0xF006_0203, 0x0, 0x0, 0x0,]),
        );
    }

    #[test]
    fn protocol() {
        assert_eq!(
            StreamConfigurationNotification::try_from(&[0xF006_0203][..])
                .unwrap()
                .protocol(),
            0x2
        );
    }

    #[test]
    fn receive_jr_timestamps() {
        assert_eq!(
            StreamConfigurationNotification::try_from(&[0xF006_0203][..])
                .unwrap()
                .receive_jr_timestamps(),
            true
        );
    }

    #[test]
    fn send_jr_timestamps() {
        assert_eq!(
            StreamConfigurationNotification::try_from(&[0xF006_0203][..])
                .unwrap()
                .send_jr_timestamps(),
            true
        );
    }
}
