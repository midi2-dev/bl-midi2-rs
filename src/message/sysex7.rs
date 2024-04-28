use crate::{
    message::{common_properties, helpers as message_helpers},
    numeric_types,
    util::BitOps,
};

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x3;

#[midi2_proc::generate_message(MinSizeUmp(2), MinSizeBytes(2))]
struct Sysex7 {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(common_properties::GroupProperty)]
    group: crate::numeric_types::u4,
    #[property(Sysex7BytesBeginByte)]
    bytes_begin_byte: (),
    #[property(Sysex7BytesEndByte)]
    bytes_end_byte: (),
    #[property(ConsistentGroups)]
    consistent_groups: (),
    #[property(ConsistentStatuses)]
    consistent_statuses: (),
    #[property(ValidPacketSizes)]
    valid_packet_sizes: (),
}

const ERR_NO_BEGIN_BYTE: &str = "Sysex messages should begin 0xF0";
const ERR_NO_END_BYTE: &str = "Sysex messages should end 0xF7";
const ERR_INVALID_PACKET_SIZE: &str = "Size field can not exceed 6";

struct Sysex7BytesBeginByte;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for Sysex7BytesBeginByte {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                if buffer.specialise_u8()[0] != 0xF0 {
                    Err(crate::error::Error::InvalidData(ERR_NO_BEGIN_BYTE))
                } else {
                    Ok(())
                }
            }
            crate::buffer::UNIT_ID_U32 => Ok(()),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                buffer.specialise_u8_mut()[0] = 0xF0;
                Ok(())
            }
            crate::buffer::UNIT_ID_U32 => Ok(()),
            _ => unreachable!(),
        }
    }
    fn default() -> Self::Type {
        ()
    }
}

struct Sysex7BytesEndByte;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for Sysex7BytesEndByte {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                let last = buffer.buffer().len() - 1;
                if buffer.specialise_u8()[last] != 0xF7 {
                    Err(crate::error::Error::InvalidData(ERR_NO_END_BYTE))
                } else {
                    Ok(())
                }
            }
            crate::buffer::UNIT_ID_U32 => Ok(()),
            _ => unreachable!(),
        }
    }
    fn write(buffer: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                let last = buffer.buffer().len() - 1;
                buffer.specialise_u8_mut()[last] = 0xF7;
                Ok(())
            }
            crate::buffer::UNIT_ID_U32 => Ok(()),
            _ => unreachable!(),
        }
    }
    fn default() -> Self::Type {
        ()
    }
}

struct ConsistentGroups;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for ConsistentGroups {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID == crate::buffer::UNIT_ID_U32 {
            message_helpers::sysex_group_consistent_groups(buffer.specialise_u32(), 2)?;
        }
        Ok(())
    }
    fn write(_: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

impl<B: crate::buffer::Buffer> crate::traits::Size<B> for Sysex7<B> {
    fn size(&self) -> usize {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                self.0
                    .specialise_u8()
                    .iter()
                    .position(|b| *b == 0xF7)
                    .expect("Message is in an invalid state. No end byte.")
                    + 1
            }
            crate::buffer::UNIT_ID_U32 => {
                self.0
                    .specialise_u32()
                    .chunks_exact(2)
                    .position(|p| {
                        let status: u8 = p[0].nibble(2).into();
                        status == 0x0 || status == 0x3
                    })
                    .expect("Message is in an invalid state. Couldn't find end packet.")
                    * 2
                    + 2
            }
            _ => unreachable!(),
        }
    }
}

struct ConsistentStatuses;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for ConsistentStatuses {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID == crate::buffer::UNIT_ID_U32 {
            message_helpers::validate_sysex_group_statuses(
                buffer.specialise_u32(),
                |s| s == numeric_types::u4::new(0x0),
                |s| s == numeric_types::u4::new(0x1),
                |s| s == numeric_types::u4::new(0x2),
                |s| s == numeric_types::u4::new(0x3),
                2,
            )?;
        }
        Ok(())
    }
    fn write(_: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

struct ValidPacketSizes;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for ValidPacketSizes {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        if <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID == crate::buffer::UNIT_ID_U32 {
            if buffer
                .specialise_u32()
                .chunks_exact(2)
                .any(|p| u8::from(p[0].nibble(3)) > 6)
            {
                Err(crate::Error::InvalidData(ERR_INVALID_PACKET_SIZE))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
    fn write(_: &mut B, _: Self::Type) -> crate::result::Result<()>
    where
        B: crate::buffer::BufferMut,
    {
        Ok(())
    }
    fn default() -> Self::Type {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::{
    //     numeric_types::*,
    //     traits::{
    //         Channeled, Data, FromBytes, FromUmp, Grouped, RebufferInto, TryFromBytes, TryFromUmp,
    //     },
    // };
    use pretty_assertions::assert_eq;

    #[test]
    fn new_bytes() {
        let message = Sysex7::<std::vec::Vec<u8>>::new();
        assert_eq!(message, Sysex7(std::vec![0xF0, 0xF7]));
    }

    #[test]
    fn data_bytes() {
        let message = Sysex7::<std::vec::Vec<u8>>::new();
        assert_eq!(message.data(), &[0xF0, 0xF7]);
    }

    #[test]
    fn try_from_bytes() {
        assert_eq!(
            Sysex7::try_from(&[0xF0_u8, 0x0_u8, 0x1_u8, 0x2_u8, 0xF7_u8][..]),
            Ok(Sysex7(&[0xF0, 0x0, 0x1, 0x2, 0xF7][..])),
        )
    }

    #[test]
    fn try_from_bytes_with_no_end_byte() {
        assert_eq!(
            Sysex7::try_from(&[0xF0_u8, 0x0_u8, 0x1_u8, 0x2_u8][..]),
            Err(crate::Error::InvalidData(ERR_NO_END_BYTE))
        )
    }

    #[test]
    fn try_from_bytes_with_no_begin_byte() {
        assert_eq!(
            Sysex7::try_from(&[0x0_u8, 0x1_u8, 0x2_u8, 0xF7_u8][..]),
            Err(crate::Error::InvalidData(ERR_NO_BEGIN_BYTE))
        )
    }

    #[test]
    fn new_ump() {
        let message = Sysex7::<std::vec::Vec<u32>>::new();
        assert_eq!(message, Sysex7(std::vec![0x3000_0000, 0x0000_0000,]));
    }

    #[test]
    fn data_ump() {
        let message = Sysex7::<std::vec::Vec<u32>>::new();
        assert_eq!(message.data(), &[0x3000_0000, 0x0000_0000,]);
    }

    #[test]
    fn try_from_ump() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Ok(Sysex7(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ))
        );
    }

    #[test]
    fn try_from_ump_inconsistent_groups() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3326_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Err(crate::Error::InvalidData(
                message_helpers::ERR_INCONSISTENT_GROUPS
            )),
        );
    }

    #[test]
    fn try_from_ump_incorrect_end_status() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3403_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Err(crate::Error::InvalidData(
                message_helpers::ERR_SYSEX_EXPECTED_END
            )),
        );
    }

    #[test]
    fn try_from_ump_incorrect_complete_status() {
        assert_eq!(
            Sysex7::try_from(&[0x3416_0001_u32, 0x0203_0405_u32,][..]),
            Err(crate::Error::InvalidData(
                message_helpers::ERR_SYSEX_EXPECTED_COMPLETE
            )),
        );
    }

    #[test]
    fn try_from_ump_incorrect_begin_status() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3406_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Err(crate::Error::InvalidData(
                message_helpers::ERR_SYSEX_EXPECTED_BEGIN
            )),
        );
    }

    #[test]
    fn try_from_ump_incorrect_continue_status() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3456_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Err(crate::Error::InvalidData(
                message_helpers::ERR_SYSEX_EXPECTED_CONTINUE
            )),
        );
    }

    #[test]
    fn try_from_ump_invalid_packet_sizes() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3427_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                ][..]
            ),
            Err(crate::Error::InvalidData(ERR_INVALID_PACKET_SIZE)),
        );
    }
}
