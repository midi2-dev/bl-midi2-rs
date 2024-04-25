use crate::{
    buffer::{UnitPrivate, UNIT_ID_U32, UNIT_ID_U8},
    message::{
        common_properties::{
            ChannelProperty, ChannelVoiceStatusProperty, GroupProperty, UmpMessageTypeProperty,
        },
        midi1_channel_voice::UMP_MESSAGE_TYPE,
    },
    numeric_types::*,
    util::BitOps,
};

const STATUS: u8 = 0b1101;

#[midi2_proc::generate_message(FixedSize, MinSizeUmp(1), MinSizeBytes(2))]
struct ChannelPressure {
    #[property(UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ChannelVoiceStatusProperty<STATUS>)]
    status: (),
    #[property(ChannelProperty)]
    channel: u4,
    #[property(GroupProperty)]
    group: u4,
    #[property(PressureProperty)]
    pressure: u7,
}

struct PressureProperty;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for PressureProperty {
    type Type = u7;
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer()[0].specialise_u32();
                Ok(b.septet(2))
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer()[1].specialise_u8();
                Ok(b.septet(0))
            }
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, v: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as UnitPrivate>::UNIT_ID {
            UNIT_ID_U32 => {
                let b = buffer.buffer_mut()[0].specialise_u32_mut();
                b.set_septet(2, v);
            }
            UNIT_ID_U8 => {
                let b = buffer.buffer_mut()[1].specialise_u8_mut();
                b.set_septet(0, v);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn default() -> Self::Type {
        u7::new(0x0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Channeled, Data, Grouped};
    use pretty_assertions::assert_eq;

    #[test]
    fn setters() {
        let mut message = ChannelPressure::new();
        message.set_group(u4::new(0xF));
        message.set_channel(u4::new(0x6));
        message.set_pressure(u7::new(0x09));
        assert_eq!(message, ChannelPressure([0x2FD6_0900]));
    }

    #[test]
    fn setters_bytes() {
        let mut message = ChannelPressure::new_bytes();
        message.set_channel(u4::new(0x6));
        message.set_pressure(u7::new(0x09));
        assert_eq!(message, ChannelPressure([0xD6, 0x09]));
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
}
