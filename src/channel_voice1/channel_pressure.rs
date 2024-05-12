use crate::{
    channel_voice1::UMP_MESSAGE_TYPE,
    detail::{common_properties, schema},
};

pub(crate) const STATUS: u8 = 0b1101;

#[midi2_proc::generate_message(
    Via(crate::channel_voice1::ChannelVoice1),
    FixedSize,
    MinSizeUmp(1),
    MinSizeBytes(2)
)]
struct ChannelPressure {
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
    pressure: crate::ux::u7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        traits::{
            Channeled, Data, FromBytes, FromUmp, Grouped, RebufferInto, TryFromBytes, TryFromUmp,
        },
        ux::*,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn new_arr() {
        assert_eq!(
            ChannelPressure::<[u32; 1]>::new(),
            ChannelPressure([0x20D0_0000])
        );
    }

    #[test]
    fn new_arr_bytes() {
        assert_eq!(
            ChannelPressure::<[u8; 2]>::new(),
            ChannelPressure([0xD0, 0x00])
        );
    }

    #[test]
    fn setters() {
        let mut message = ChannelPressure::<[u32; 4]>::new();
        message.set_group(u4::new(0xF));
        message.set_channel(u4::new(0x6));
        message.set_pressure(u7::new(0x09));
        assert_eq!(message, ChannelPressure([0x2FD6_0900, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = ChannelPressure::<[u8; 3]>::new();
        message.set_channel(u4::new(0x6));
        message.set_pressure(u7::new(0x09));
        assert_eq!(message, ChannelPressure([0xD6, 0x09, 0x0]));
    }

    #[test]
    fn group() {
        assert_eq!(
            ChannelPressure::try_from(&[0x2FD6_0900][..])
                .unwrap()
                .group(),
            u4::new(0xF),
        );
    }

    #[test]
    fn from_outsized_data() {
        assert_eq!(
            ChannelPressure::try_from(&[0x20D0_0000_u32, 0x0_u32][..]),
            Ok(ChannelPressure(&[0x20D0_0000_u32, 0x0_u32][..])),
        );
    }

    #[test]
    fn from_empty_data() {
        assert_eq!(
            ChannelPressure::try_from(&<[u32; 0] as Default>::default()[..]),
            Err(crate::error::Error::InvalidData("Slice is too short")),
        );
    }

    #[test]
    fn channel() {
        assert_eq!(
            ChannelPressure::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn pressure() {
        assert_eq!(
            ChannelPressure::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .pressure(),
            u7::new(0x09),
        );
    }

    #[test]
    fn channel_bytes() {
        assert_eq!(
            ChannelPressure::try_from(&[0xD6_u8, 0x09_u8][..])
                .unwrap()
                .channel(),
            u4::new(0x6),
        );
    }

    #[test]
    fn pressure_bytes() {
        assert_eq!(
            ChannelPressure::try_from(&[0xD6_u8, 0x09_u8][..])
                .unwrap()
                .pressure(),
            u7::new(0x09),
        );
    }

    #[test]
    fn data() {
        assert_eq!(
            ChannelPressure::try_from(&[0x2FD6_0900_u32][..])
                .unwrap()
                .data(),
            &[0x2FD6_0900]
        );
    }

    #[test]
    fn data_bytes() {
        assert_eq!(
            ChannelPressure::try_from(&[0xD6_u8, 0x09_u8][..])
                .unwrap()
                .data(),
            &[0xD6_u8, 0x09_u8]
        );
    }

    #[test]
    fn data_with_outsized_buffer() {
        assert_eq!(ChannelPressure::<[u32; 2]>::new().data(), &[0x20D0_0000]);
    }

    #[test]
    fn from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u32>>::from_bytes(borrowed),
            ChannelPressure(std::vec![0x20D6_0900_u32]),
        );
    }

    #[test]
    fn from_ump() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u8>>::from_ump(borrowed),
            ChannelPressure(std::vec![0xD6_u8, 0x09_u8]),
        );
    }

    #[test]
    fn try_from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u32; 1]>::try_from_bytes(borrowed),
            Ok(ChannelPressure([0x20D6_0900_u32])),
        );
    }

    #[test]
    fn try_from_ump() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u8; 2]>::try_from_ump(borrowed),
            Ok(ChannelPressure([0xD6_u8, 0x09_u8])),
        );
    }

    #[test]
    fn new_with_custom_buffer() {
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u32>>::new(),
            ChannelPressure::<[u32; 4]>::new().rebuffer_into(),
        )
    }

    #[test]
    fn new_with_custom_buffer_bytes() {
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u8>>::new(),
            ChannelPressure(std::vec![0xD0, 0x0])
        )
    }

    #[test]
    fn new_with_arr() {
        assert_eq!(
            ChannelPressure::<[u32; 2]>::new(),
            ChannelPressure([0x20D0_0000, 0x0]),
        )
    }

    #[test]
    fn new_with_arr_bytes() {
        assert_eq!(
            ChannelPressure::<[u8; 3]>::new(),
            ChannelPressure([0xD0, 0x00, 0x0]),
        )
    }

    #[test]
    fn try_new_with_custom_buffer_fail() {
        assert_eq!(
            ChannelPressure::<[u32; 0]>::try_new(),
            Err(crate::error::BufferOverflow),
        )
    }

    #[test]
    fn try_new_with_custom_buffer_bytes_fail() {
        assert_eq!(
            ChannelPressure::<[u8; 1]>::try_new(),
            Err(crate::error::BufferOverflow),
        )
    }
}

#[cfg(test)]
mod rebuffer_tests {
    use super::*;
    use crate::traits::{RebufferFrom, RebufferInto, TryRebufferFrom, TryRebufferInto};
    use pretty_assertions::assert_eq;

    #[test]
    fn rebuffer_from() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u32>>::rebuffer_from(borrowed),
            ChannelPressure(std::vec![0x2FD6_0900_u32]),
        );
    }

    #[test]
    fn rebuffer_from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<std::vec::Vec<u8>>::rebuffer_from(borrowed),
            ChannelPressure(std::vec![0xD6_u8, 0x09_u8]),
        );
    }

    #[test]
    fn try_rebuffer_from() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u32; 1]>::try_rebuffer_from(borrowed),
            Ok(ChannelPressure([0x2FD6_0900_u32])),
        );
    }

    #[test]
    fn try_rebuffer_from_fail() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u32; 0]>::try_rebuffer_from(borrowed),
            Err(crate::error::BufferOverflow),
        );
    }

    #[test]
    fn try_rebuffer_from_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u8; 2]>::try_rebuffer_from(borrowed),
            Ok(ChannelPressure([0xD6_u8, 0x09_u8])),
        );
    }

    #[test]
    fn try_rebuffer_from_bytes_fail() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        assert_eq!(
            ChannelPressure::<[u8; 0]>::try_rebuffer_from(borrowed),
            Err(crate::error::BufferOverflow),
        );
    }

    #[test]
    fn rebuffer_into() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: ChannelPressure<std::vec::Vec<u32>> = borrowed.rebuffer_into();
        assert_eq!(owned, ChannelPressure(std::vec![0x2FD6_0900_u32]),);
    }

    #[test]
    fn rebuffer_into_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: ChannelPressure<std::vec::Vec<u8>> = borrowed.rebuffer_into();
        assert_eq!(owned, ChannelPressure(std::vec![0xD6_u8, 0x09_u8]),);
    }

    #[test]
    fn try_rebuffer_into() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: core::result::Result<ChannelPressure<[u32; 1]>, crate::error::BufferOverflow> =
            borrowed.try_rebuffer_into();
        assert_eq!(owned, Ok(ChannelPressure([0x2FD6_0900_u32])));
    }

    #[test]
    fn try_rebuffer_into_bytes() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: core::result::Result<ChannelPressure<[u8; 2]>, crate::error::BufferOverflow> =
            borrowed.try_rebuffer_into();
        assert_eq!(owned, Ok(ChannelPressure([0xD6_u8, 0x09_u8])));
    }

    #[test]
    fn try_rebuffer_into_fail() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: core::result::Result<ChannelPressure<[u32; 0]>, crate::error::BufferOverflow> =
            borrowed.try_rebuffer_into();
        assert_eq!(owned, Err(crate::error::BufferOverflow));
    }

    #[test]
    fn try_rebuffer_into_bytes_fail() {
        let buffer = [0xD6_u8, 0x09_u8];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let owned: core::result::Result<ChannelPressure<[u8; 0]>, crate::error::BufferOverflow> =
            borrowed.try_rebuffer_into();
        assert_eq!(owned, Err(crate::error::BufferOverflow));
    }

    #[test]
    fn clone() {
        let message = ChannelPressure::<[u32; 4]>::new();
        let clone = message.clone();
        assert_eq!(message, clone);
    }

    #[test]
    fn clone_borrowed() {
        let buffer = [0x2FD6_0900_u32];
        let borrowed = ChannelPressure::try_from(&buffer[..]).unwrap();
        let clone = borrowed.clone();
        assert_eq!(borrowed, clone);
    }
}
