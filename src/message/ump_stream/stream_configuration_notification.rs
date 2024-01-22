use crate::message::ump_stream::TYPE_CODE as UMP_STREAM_TYPE;
const STATUS: u32 = 0x6;

#[midi2_proc::generate_message()]
struct StreamConfigurationNotification {
    ump_type:
        Property<NumericalConstant<UMP_STREAM_TYPE>, UmpSchema<0xF000_0000, 0x0, 0x0, 0x0>, ()>,
    format: Property<NumericalConstant<0x0>, UmpSchema<0x0C00_0000, 0x0, 0x0, 0x0>, ()>,
    status: Property<NumericalConstant<STATUS>, UmpSchema<0x03FF_0000, 0x0, 0x0, 0x0>, ()>,
    protocol: Property<u8, UmpSchema<0x0000_FF00, 0x0, 0x0, 0x0>, ()>,
    receive_jr_timestamps:
        Property<bool, UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0010, 0x0, 0x0, 0x0>, ()>,
    send_jr_timestamps:
        Property<bool, UmpSchema<0b0000_0000_0000_0000_0000_0000_0000_0001, 0x0, 0x0, 0x0>, ()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn builder() {
        assert_eq!(
            StreamConfigurationNotificationMessage::builder()
                .protocol(0x2)
                .receive_jr_timestamps(true)
                .send_jr_timestamps(true)
                .build(),
            Ok(StreamConfigurationNotificationMessage::Owned(
                StreamConfigurationNotificationOwned([0xF006_0203, 0x0, 0x0, 0x0,])
            )),
        );
    }

    #[test]
    fn protocol() {
        assert_eq!(
            StreamConfigurationNotificationMessage::from_data(&[0xF006_0203])
                .unwrap()
                .protocol(),
            0x2
        );
    }

    #[test]
    fn receive_jr_timestamps() {
        assert_eq!(
            StreamConfigurationNotificationMessage::from_data(&[0xF006_0203])
                .unwrap()
                .receive_jr_timestamps(),
            true
        );
    }

    #[test]
    fn send_jr_timestamps() {
        assert_eq!(
            StreamConfigurationNotificationMessage::from_data(&[0xF006_0203])
                .unwrap()
                .send_jr_timestamps(),
            true
        );
    }
}
