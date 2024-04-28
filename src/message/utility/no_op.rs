use crate::message::{
    common_properties::{ChannelVoiceStatusProperty, UmpMessageTypeProperty},
    utility::UMP_MESSAGE_TYPE,
};

const STATUS: u8 = 0x0;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1))]
struct NoOp {
    #[property(UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ChannelVoiceStatusProperty<STATUS>)]
    status: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(NoOp::new_arr(), NoOp([0x0, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn from_data() {
        assert_eq!(
            NoOp::try_from(&[0x0000_0000][..]),
            Ok(NoOp(&[0x0000_0000][..]))
        );
    }
}
