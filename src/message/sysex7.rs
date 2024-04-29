use crate::{
    message::{common_properties, helpers as message_helpers},
    numeric_types::{self, u7},
    traits::{Sysex, SysexInternal},
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

const START_BYTE: u8 = 0xF0;
const END_BYTE: u8 = 0xF7;

struct Sysex7BytesBeginByte;

impl<B: crate::buffer::Buffer> crate::util::property::Property<B> for Sysex7BytesBeginByte {
    type Type = ();
    fn read(buffer: &B) -> crate::result::Result<Self::Type> {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                if buffer.specialise_u8()[0] != START_BYTE {
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
                buffer.specialise_u8_mut()[0] = START_BYTE;
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
            crate::buffer::UNIT_ID_U8 => buffer
                .specialise_u8()
                .iter()
                .position(|b| *b == 0xF7)
                .map(|_| ())
                .ok_or(crate::error::Error::InvalidData(ERR_NO_END_BYTE)),
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
                buffer.specialise_u8_mut()[last] = END_BYTE;
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
            message_helpers::sysex_group_consistent_groups(
                buffer.specialise_u32(),
                2,
                crate::numeric_types::u4::new(UMP_MESSAGE_TYPE),
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
                crate::numeric_types::u4::new(UMP_MESSAGE_TYPE),
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

fn next_bytes(index: &mut usize, data: &[u8]) -> Option<u7> {
    if *index == data.len() {
        None
    } else {
        let ret = Some(u7::new(data[*index]));
        *index += 1;
        ret
    }
}

#[derive(Debug, Clone)]
pub struct PayloadIterator<'a, U: crate::buffer::Unit> {
    data: &'a [U],
    payload_index: usize,
    // unused in bytes mode
    packet_index: usize,
}

impl<'a, U: crate::buffer::Unit> core::iter::Iterator for PayloadIterator<'a, U> {
    type Item = numeric_types::u7;
    fn next(&mut self) -> Option<Self::Item> {
        match U::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => next_bytes(
                &mut self.payload_index,
                <U as crate::buffer::UnitPrivate>::specialise_buffer_u8(self.data),
            ),
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match U::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                todo!()
            }
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        match U::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                todo!()
            }
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match U::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => (self.len(), Some(self.len())),
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl<'a, U: crate::buffer::Unit> core::iter::FusedIterator for PayloadIterator<'a, U> {}

impl<'a, U: crate::buffer::Unit> core::iter::ExactSizeIterator for PayloadIterator<'a, U> {
    fn len(&self) -> usize {
        match U::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => self.data[self.payload_index..].len(),
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl<B: crate::buffer::Buffer> Sysex<B> for Sysex7<B> {
    type Byte = numeric_types::u7;
    type PayloadIterator<'a> = PayloadIterator<'a, B::Unit>
    where
        B::Unit: 'a,
        Self: 'a;

    fn payload<'a>(&'a self) -> Self::PayloadIterator<'a>
    where
        B::Unit: 'a,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => PayloadIterator {
                data: &self.0.buffer()[1..self.size() - 1],
                payload_index: 0,
                packet_index: 0,
            },
            crate::buffer::UNIT_ID_U32 => PayloadIterator {
                data: &self.0.buffer()[..self.size()],
                payload_index: 0,
                packet_index: 0,
            },
            _ => unreachable!(),
        }
    }

    fn set_payload<D>(&mut self, data: D)
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        message_helpers::set_sysex_data(self, data)
    }

    fn try_set_payload<D>(
        &mut self,
        data: D,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        D: core::iter::Iterator<Item = Self::Byte>,
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        message_helpers::try_set_sysex_data(self, data)
    }
}

impl<B: crate::buffer::Buffer> SysexInternal<B> for Sysex7<B> {
    fn resize(&mut self, payload_size: usize)
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                let buffer_sz = payload_size + 2;
                let old_payload_size = self.payload_size();
                self.0.resize(buffer_sz);
                if payload_size > old_payload_size {
                    // erase old end bit
                    self.0.specialise_u8_mut()[old_payload_size + 1] = 0;
                }
                self.0.specialise_u8_mut()[buffer_sz - 1] = END_BYTE;
            }
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }

    fn try_resize(
        &mut self,
        payload_size: usize,
    ) -> core::result::Result<(), crate::error::BufferOverflow>
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                let old_payload_size = self.payload_size();
                let mut buffer_sz = payload_size + 2;
                let resize_result = self.0.try_resize(buffer_sz);
                if resize_result.is_err() {
                    buffer_sz = self.0.buffer().len();
                }
                if buffer_sz > old_payload_size {
                    // erase old end bit
                    self.0.specialise_u8_mut()[old_payload_size + 1] = 0;
                }
                self.0.specialise_u8_mut()[buffer_sz - 1] = END_BYTE;
                resize_result
            }
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }

    fn write_datum(&mut self, datum: Self::Byte, payload_index: usize)
    where
        B: crate::buffer::BufferMut,
    {
        match <B::Unit as crate::buffer::UnitPrivate>::UNIT_ID {
            crate::buffer::UNIT_ID_U8 => {
                self.0.specialise_u8_mut()[payload_index + 1] = datum.into();
            }
            crate::buffer::UNIT_ID_U32 => {
                todo!()
            }
            _ => unreachable!(),
        }
    }

    fn payload_size(&self) -> usize {
        self.payload().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{numeric_types::*, traits::Sysex};
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
    fn try_from_oversized_bytes() {
        assert_eq!(
            Sysex7::try_from(&[0xF0_u8, 0x0_u8, 0x1_u8, 0x2_u8, 0xF7, 0x0][..]),
            Ok(Sysex7(&[0xF0, 0x0, 0x1, 0x2, 0xF7, 0x0][..])),
        )
    }

    #[test]
    fn data_oversized_bytes() {
        assert_eq!(
            Sysex7::try_from(&[0xF0_u8, 0x0_u8, 0x1_u8, 0x2_u8, 0xF7, 0x0][..])
                .unwrap()
                .data(),
            &[0xF0, 0x0, 0x1, 0x2, 0xF7],
        );
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
    fn try_from_oversized_ump() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                    0x0000_0000_u32,
                    0x0000_0000_u32,
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
                    0x0000_0000_u32,
                    0x0000_0000_u32,
                ][..]
            ))
        );
    }

    #[test]
    fn data_oversized_ump() {
        assert_eq!(
            Sysex7::try_from(
                &[
                    0x3416_0001_u32,
                    0x0203_0405_u32,
                    0x3426_0607_u32,
                    0x0809_0A0B_u32,
                    0x3433_0C0D_u32,
                    0x0E00_0000_u32,
                    0x0000_0000_u32,
                    0x0000_0000_u32,
                ][..]
            )
            .unwrap()
            .data(),
            &[
                0x3416_0001,
                0x0203_0405,
                0x3426_0607,
                0x0809_0A0B,
                0x3433_0C0D,
                0x0E00_0000,
            ],
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

    #[test]
    fn set_payload_bytes() {
        let mut message = Sysex7::<std::vec::Vec<u8>>::new();
        message.set_payload((0u8..20u8).map(u7::new));
        assert_eq!(
            message,
            Sysex7(std::vec![
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0xF7,
            ])
        );
    }

    #[test]
    fn reset_smaller_payload_bytes() {
        let mut message = Sysex7::<std::vec::Vec<u8>>::new();
        message.set_payload((0u8..20u8).map(u7::new));
        message.set_payload((0u8..10).map(u7::new));
        assert_eq!(
            message,
            Sysex7(std::vec![
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0xF7,
            ])
        );
    }

    #[test]
    fn reset_larger_payload_bytes() {
        let mut message = Sysex7::<std::vec::Vec<u8>>::new();
        message.set_payload((0u8..20u8).map(u7::new));
        message.set_payload((0u8..30).map(u7::new));
        assert_eq!(
            message,
            Sysex7(std::vec![
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
                0x1B, 0x1C, 0x1D, 0xF7,
            ])
        );
    }

    #[test]
    fn set_rubbish_payload_bytes() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        let mut message = Sysex7::<std::vec::Vec<u8>>::new();
        message.set_payload(RubbishPayloadIterator::new().map(u7::new));
        assert_eq!(
            message,
            Sysex7(std::vec![
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
                0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
                0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0xF7,
            ])
        );
    }

    #[test]
    fn try_set_rubbish_payload_to_fixed_size_buffer() {
        use crate::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        let mut message = Sysex7::<[u8; 52]>::try_new().unwrap();
        message
            .try_set_payload(RubbishPayloadIterator::new().map(u7::new))
            .expect("Shouldn't fail");
        assert_eq!(
            message,
            Sysex7([
                0xF0, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A,
                0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
                0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0xF7,
            ])
        );
    }

    #[test]
    fn payload_bytes() {
        assert_eq!(
            std::vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D,
            ],
            Sysex7::try_from(
                &[
                    0xF0_u8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                    0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                    0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0xF7,
                ][..]
            )
            .unwrap()
            .payload()
            .map(u8::from)
            .collect::<std::vec::Vec<u8>>()
        );
    }
}
