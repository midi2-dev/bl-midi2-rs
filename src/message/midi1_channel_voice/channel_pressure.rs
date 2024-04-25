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
    use crate::traits::{Channeled, Grouped};
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

    // #[test]
    // fn group() {
    //     assert_eq!(
    //         ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
    //             .unwrap()
    //             .group(),
    //         u4::new(0xF),
    //     );
    // }
    //
    // #[test]
    // fn channel() {
    //     assert_eq!(
    //         ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
    //             .unwrap()
    //             .channel(),
    //         u4::new(0x6),
    //     );
    // }
    //
    // #[test]
    // fn pressure() {
    //     assert_eq!(
    //         ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
    //             .unwrap()
    //             .pressure(),
    //         u7::new(0x09),
    //     );
    // }
    //
    // #[test]
    // fn into_owned() {
    //     assert_eq!(
    //         ChannelPressureMessage::from_data(&[0x2FD6_0900, 0x0, 0x0, 0x0])
    //             .unwrap()
    //             .into_owned(),
    //         ChannelPressureOwned::builder()
    //             .group(u4::new(0xF))
    //             .channel(u4::new(0x6))
    //             .pressure(u7::new(0x09))
    //             .build()
    //             .unwrap(),
    //     );
    // }
}
