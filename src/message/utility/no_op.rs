use crate::message::{
    common_properties::{UmpMessageTypeProperty, UtilityStatusProperty},
    utility::UMP_MESSAGE_TYPE,
};

const STATUS: u8 = 0x0;

#[midi2_proc::generate_message(Ump, FixedSizeUmp(1))]
struct NoOp {
    #[property(UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    #[ump]
    #[constant]
    ump_type: crate::u4,
    #[property(UtilityStatusProperty<STATUS>)]
    #[ump]
    #[constant]
    status: crate::u4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_group() {
        let mut message = NoOp::new();
        assert_eq!(message, NoOp([0x0000_0000, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn group() {
        assert_eq!(
            NoOp::try_from(&[0x0000_0000][..]),
            Ok(NoOp(&[0x0000_0000][..]))
        );
    }
}
