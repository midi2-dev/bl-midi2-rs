use crate::{
    detail::{common_properties, helpers as message_helpers, BitOps},
    traits::{Sysex, SysexInternal},
    ux,
};

mod packet;

pub use packet::Packet;

pub(crate) const UMP_MESSAGE_TYPE: u8 = 0x5;

const ERR_INVALID_NUMBER_OF_PAYLOAD_BYTES: &str = "Invalid number of payload bytes in packet";
const ERR_INCONSISTENT_STREAM_ID: &str = "Inconsistent stream id fields across packets";

#[midi2_proc::generate_message(MinSizeUmp(4))]
/// A semantic wrapper type around MIDI 2.0 System Exclusive 8bit data.
/// See the [module docs](crate::sysex8) for more detailed info
struct Sysex8 {
    #[property(common_properties::UmpMessageTypeProperty<UMP_MESSAGE_TYPE>)]
    ump_type: (),
    #[property(ConsistentStatuses)]
    #[readonly]
    consistent_statuses: (),
    #[property(ValidPacketSizes)]
    valid_packet_sizes: (),
    #[property(GroupProperty)]
    group: crate::ux::u4,
    #[property(StreamIdProperty)]
    stream_id: u8,
    #[property(SysexPayloadPlaceholder)]
    #[readonly]
    #[writeonly]
    sysex_payload: (),
}

struct ConsistentStatuses;

impl<B: crate::buffer::Ump> crate::detail::property::Property<B> for ConsistentStatuses {
    type Type = ();
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B>
    for ConsistentStatuses
{
    fn read(_buffer: &'a B) -> Self::Type {}
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        message_helpers::validate_sysex_group_statuses(
            buffer.buffer(),
            |p| u8::from(p[0].nibble(2)) == 0x0,
            |p| u8::from(p[0].nibble(2)) == 0x1,
            |p| u8::from(p[0].nibble(2)) == 0x2,
            |p| u8::from(p[0].nibble(2)) == 0x3,
            4,
            crate::ux::u4::new(UMP_MESSAGE_TYPE),
        )
    }
}

struct ValidPacketSizes;

impl<B: crate::buffer::Ump> crate::detail::property::Property<B> for ValidPacketSizes {
    type Type = ();
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B> for ValidPacketSizes {
    fn read(_buffer: &'a B) -> Self::Type {}
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        if buffer.buffer().chunks_exact(4).any(|p| {
            let number_bytes = u8::from(p[0].nibble(3));
            !(1..=14).contains(&number_bytes)
        }) {
            Err(crate::error::InvalidData(
                ERR_INVALID_NUMBER_OF_PAYLOAD_BYTES,
            ))
        } else {
            Ok(())
        }
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for ValidPacketSizes
{
    fn write(buffer: &mut B, _: Self::Type) {
        for packet in buffer
            .buffer_mut()
            .chunks_exact_mut(4)
            .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
        {
            let sz = packet[0].nibble(3);
            packet[0].set_nibble(3, sz.max(ux::u4::new(1)));
        }
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

struct GroupProperty;

impl<B: crate::buffer::Ump> crate::detail::property::Property<B> for GroupProperty {
    type Type = ux::u4;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B> for GroupProperty {
    fn read(buffer: &'a B) -> Self::Type {
        buffer.buffer()[0].nibble(1)
    }
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        message_helpers::sysex_group_consistent_groups(
            buffer.buffer(),
            4,
            crate::ux::u4::new(UMP_MESSAGE_TYPE),
        )
    }
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for GroupProperty
{
    fn write(buffer: &mut B, group: Self::Type) {
        for packet in buffer
            .buffer_mut()
            .chunks_exact_mut(4)
            .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
        {
            packet[0].set_nibble(1, group);
        }
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

struct StreamIdProperty;

impl<B: crate::buffer::Ump> crate::detail::property::Property<B> for StreamIdProperty {
    type Type = u8;
}

impl<'a, B: crate::buffer::Ump> crate::detail::property::ReadProperty<'a, B> for StreamIdProperty {
    fn read(buffer: &'a B) -> Self::Type {
        stream_id_from_packet(buffer.buffer())
    }
    fn validate(buffer: &B) -> Result<(), crate::error::InvalidData> {
        let sid = stream_id_from_packet;
        let buffer = buffer.buffer();
        if buffer
            .chunks_exact(4)
            .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
            .all(|chunk| sid(chunk) == sid(buffer))
        {
            Ok(())
        } else {
            Err(crate::error::InvalidData(ERR_INCONSISTENT_STREAM_ID))
        }
    }
}

fn stream_id_from_packet(packet: &[u32]) -> u8 {
    packet[0].octet(2)
}

impl<B: crate::buffer::Ump + crate::buffer::BufferMut> crate::detail::property::WriteProperty<B>
    for StreamIdProperty
{
    fn write(buffer: &mut B, id: Self::Type) {
        for packet in buffer
            .buffer_mut()
            .chunks_exact_mut(4)
            .take_while(|packet| u8::from(packet[0].nibble(0)) == UMP_MESSAGE_TYPE)
        {
            packet[0].set_octet(3, id);
        }
    }
    fn validate(_v: &Self::Type) -> Result<(), crate::error::InvalidData> {
        Ok(())
    }
    fn default() -> Self::Type {
        Default::default()
    }
}

impl<B: crate::buffer::Ump> crate::traits::Size<B> for Sysex8<B> {
    fn size(&self) -> usize {
        self.0
            .buffer()
            .chunks_exact(4)
            .position(|p| {
                let status: u8 = p[0].nibble(2).into();
                status == 0x0 || status == 0x3
            })
            .expect("Message is in an invalid state. Couldn't find end packet.")
            * 4
            + 4
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayloadIterator<'a> {
    data: &'a [u32],
    packet_index: usize,
    payload_index: usize,
    size_cache: usize,
}

impl<'a> core::iter::Iterator for PayloadIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished() {
            return None;
        }

        // skip empty packets
        while !self.finished() && self.size_of_current_packet() == 0 {
            self.payload_index = 0;
            self.packet_index += 1;
        }

        let ret = Some(self.value());
        self.advance();
        ret
    }

    /// # Complexity
    ///
    /// O(n)
    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        let mut do_nth = || {
            let mut packets = self.data[self.packet_index * 4..]
                .chunks_exact(4)
                .enumerate();

            {
                // first we check to see whether the requested byte lies
                // within the first packet where we are potentially already offset
                let remaining = Self::packet_size(packets.next()?.1) - self.payload_index;
                if n < remaining {
                    self.payload_index += n;
                    self.size_cache -= n;
                    return self.next();
                } else {
                    n -= remaining;
                    self.size_cache -= remaining;
                }
            }

            // we then cycle through all the packets until we travelled as far as the
            // requested location
            loop {
                let (packet_index, packet) = packets.next()?;
                let size = Self::packet_size(packet);
                if n < size {
                    // we found the requested packet
                    self.packet_index += packet_index;
                    self.payload_index = n;
                    self.size_cache -= n;
                    break;
                }
                n -= size;
                self.size_cache -= size;
            }

            self.next()
        };

        let ret = do_nth();
        if ret.is_none() {
            // if we failed it means we ran out of data
            // so we set the iterator into finished state
            self.packet_index = self.data.len() / 2;
            self.size_cache = 0;
        }
        ret
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.size_cache
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_cache, Some(self.size_cache))
    }
}

impl<'a> core::iter::ExactSizeIterator for PayloadIterator<'a> {
    fn len(&self) -> usize {
        self.size_cache
    }
}

impl<'a> PayloadIterator<'a> {
    fn value(&self) -> u8 {
        let buffer_index = self.packet_index * 4 + (self.payload_index + 3) / 4;
        let octet_index = (self.payload_index + 3) % 4;
        self.data[buffer_index].octet(octet_index)
    }
    fn packet_size(packet: &[u32]) -> usize {
        u8::from(packet[0].nibble(3)) as usize - 1
    }
    fn finished(&self) -> bool {
        self.size_cache == 0
    }
    fn size_of_current_packet(&self) -> usize {
        Self::packet_size(&self.data[self.packet_index * 4..(self.packet_index * 4 + 4)])
    }
    fn advance(&mut self) {
        self.payload_index += 1;
        if !self.finished() {
            self.size_cache -= 1;
        }
        if self.payload_index >= self.size_of_current_packet() {
            // end of message
            self.packet_index += 1;
            self.payload_index = 0;
        }
    }
}

impl<B: crate::buffer::Ump> Sysex<B> for Sysex8<B> {
    type Byte = u8;
    type PayloadIterator<'a>
        = PayloadIterator<'a>
    where
        B: 'a;
    fn payload<'a>(&'a self) -> Self::PayloadIterator<'a>
    where
        <B as crate::buffer::Buffer>::Unit: 'a,
    {
        PayloadIterator {
            data: self.0.buffer(),
            packet_index: 0,
            payload_index: 0,
            size_cache: self
                .data()
                .chunks_exact(4)
                .map(PayloadIterator::packet_size)
                .sum(),
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

impl<B: crate::buffer::Ump> SysexInternal<B> for Sysex8<B> {
    fn resize(&mut self, payload_size: usize)
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferResize,
    {
        try_resize(self, payload_size, |s, sz| {
            s.0.resize(sz);
            Ok(())
        })
        .unwrap()
    }

    fn try_resize(
        &mut self,
        payload_size: usize,
    ) -> core::result::Result<(), crate::traits::SysexTryResizeError>
    where
        B: crate::buffer::BufferMut + crate::buffer::BufferTryResize,
    {
        try_resize(self, payload_size, |s, sz| s.0.try_resize(sz))
    }

    fn write_datum(&mut self, datum: Self::Byte, payload_index: usize)
    where
        B: crate::buffer::BufferMut,
    {
        // data is written into the buffer contiguously
        // meaning only the last packet may have a size < 6
        let buffer_index = 4 * (payload_index / 13);
        let byte_index = payload_index % 13;
        self.0.specialise_u32_mut()[buffer_index + (byte_index + 3) / 4]
            .set_octet((byte_index + 3) % 4, datum);
    }

    fn payload_size(&self) -> usize {
        self.payload().len()
    }
}

fn try_resize<
    B: crate::buffer::Ump + crate::buffer::BufferMut,
    ResizeBuffer: Fn(&mut Sysex8<B>, usize) -> Result<(), crate::error::BufferOverflow>,
>(
    sysex: &mut Sysex8<B>,
    mut payload_size: usize,
    try_resize_buffer: ResizeBuffer,
) -> Result<(), crate::traits::SysexTryResizeError> {
    use ux::u4;

    let mut buffer_size = buffer_size_from_payload_size(payload_size);
    let resize_result = try_resize_buffer(sysex, buffer_size);
    if resize_result.is_err() {
        // resize failed. We make do with what we've got
        buffer_size = sysex.0.buffer().len();
        payload_size = buffer_size * 13 / 4;
    }

    let mut iter = sysex
        .0
        .buffer_mut()
        .chunks_exact_mut(4)
        .take(buffer_size / 4)
        .peekable();
    let mut group = None;

    const MESSAGE_TYPE: u4 = u4::new(UMP_MESSAGE_TYPE);
    const STATUS_COMPLETE: u4 = u4::new(0x0);
    const STATUS_START: u4 = u4::new(0x1);
    const STATUS_CONTINUE: u4 = u4::new(0x2);
    const STATUS_END: u4 = u4::new(0x3);

    // first packet
    if let Some(first_packet) = iter.next() {
        first_packet[0].set_nibble(0, MESSAGE_TYPE);
        group = Some(first_packet[0].nibble(1));
        if iter.peek().is_some() {
            // start packet
            first_packet[0].set_nibble(2, STATUS_START);
            first_packet[0].set_nibble(3, u4::new(14));
        } else {
            // complete packet
            first_packet[0].set_nibble(2, STATUS_COMPLETE);
            first_packet[0].set_nibble(3, u4::new(payload_size as u8 + 1));
        }
    }

    while let Some(chunk) = iter.next() {
        chunk[0].set_nibble(0, MESSAGE_TYPE);
        chunk[0].set_nibble(1, group.unwrap());
        if iter.peek().is_some() {
            // middle packet
            chunk[0].set_nibble(2, STATUS_CONTINUE);
            chunk[0].set_nibble(3, u4::new(14));
        } else {
            // last packet
            chunk[0].set_nibble(2, STATUS_END);
            match payload_size % 13 {
                0 => {
                    chunk[0].set_nibble(3, u4::new(14));
                }
                r => {
                    chunk[0].set_nibble(3, u4::new(r as u8 + 1));
                    // zero off the end of the packet
                    for i in r..13 {
                        chunk[(i + 3) / 4].set_octet((i + 3) % 4, 0x0);
                    }
                }
            };
        }
    }

    resize_result.map_err(|_| crate::traits::SysexTryResizeError(payload_size))
}

fn buffer_size_from_payload_size(payload_size: usize) -> usize {
    if payload_size % 13 == 0 {
        if payload_size == 0 {
            4
        } else {
            payload_size * 4 / 13
        }
    } else {
        4 * (payload_size / 13 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(
            Sysex8::<std::vec::Vec<u32>>::new(),
            Sysex8(std::vec![0x5001_0000, 0x0, 0x0, 0x0])
        );
    }

    #[test]
    fn try_new_with_buffer() {
        let mut buffer = [0x0_u32; 16];
        let message = Sysex8::try_new_with_buffer(&mut buffer[..]).unwrap();
        assert_eq!(
            message.data(),
            &[0x5001_0000, 0x0000_0000, 0x0000_0000, 0x0000_0000,]
        );
    }

    #[test]
    fn try_new_with_buffer_undersized_buffer() {
        let mut buffer = [0x0_u32; 2];
        let result = Sysex8::try_new_with_buffer(&mut buffer[..]);
        assert_eq!(result, Err(crate::error::BufferOverflow));
    }

    #[test]
    fn try_new_with_buffer_write_payload() {
        let mut buffer = [0x0_u32; 16];
        let mut message = Sysex8::try_new_with_buffer(&mut buffer[..]).unwrap();
        message.try_set_payload(0..10).unwrap();
        assert_eq!(
            message.data(),
            &[0x500B_0000, 0x0102_0304, 0x0506_0708, 0x0900_0000,][..]
        );
    }

    #[test]
    fn new_with_buffer_buffer_is_zeroed() {
        let mut buffer = (0_u32..4).collect::<std::vec::Vec<u32>>();
        let _ = Sysex8::new_with_buffer(&mut buffer);
        assert_eq!(buffer, std::vec![0x5001_0000, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn new_set_group() {
        use crate::traits::Grouped;

        let mut message = Sysex8::<std::vec::Vec<u32>>::new();
        message.set_group(ux::u4::new(0xC));

        assert_eq!(message, Sysex8(std::vec![0x5C01_0000, 0x0, 0x0, 0x0]));
    }

    #[test]
    fn try_from_slice() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Ok(Sysex8(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            )),
        );
    }

    #[test]
    fn try_from_slice_invalid_packet_size() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542F_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(
                ERR_INVALID_NUMBER_OF_PAYLOAD_BYTES
            )),
        );
    }

    #[test]
    fn try_from_slice_inconsistent_groups() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x512E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(
                crate::detail::helpers::ERR_INCONSISTENT_GROUPS
            )),
        );
    }

    #[test]
    fn try_from_slice_inconsistent_stream_id() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_CC0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(ERR_INCONSISTENT_STREAM_ID,)),
        );
    }

    #[test]
    fn try_from_slice_expected_start() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x540E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_BEGIN
            )),
        );
    }

    #[test]
    fn try_from_slice_expected_continue() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x541E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_CONTINUE
            )),
        );
    }

    #[test]
    fn try_from_slice_expected_end() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x542C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            ),
            Err(crate::error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_END
            )),
        );
    }

    #[test]
    fn try_from_slice_expected_complete() {
        assert_eq!(
            Sysex8::try_from(&[0x541C_BB00, 0x0102_0304, 0x0506_0708, 0x090A_0B00,][..]),
            Err(crate::error::InvalidData(
                crate::detail::helpers::ERR_SYSEX_EXPECTED_COMPLETE
            )),
        );
    }

    #[test]
    fn group() {
        use crate::Grouped;

        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            )
            .unwrap()
            .group(),
            ux::u4::new(0x4),
        );
    }

    #[test]
    fn stream_id() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            )
            .unwrap()
            .stream_id(),
            0xBB,
        );
    }

    #[test]
    fn payload() {
        assert_eq!(
            Sysex8::try_from(
                &[
                    0x541E_BB00,
                    0x0102_0304,
                    0x0506_0708,
                    0x090A_0B0C,
                    0x542E_BB0D,
                    0x0E0F_1011,
                    0x1213_1415,
                    0x1617_1819,
                    0x542E_BB1A,
                    0x1B1C_1D1E,
                    0x1F20_2122,
                    0x2324_2526,
                    0x543C_BB27,
                    0x2829_2A2B,
                    0x2C2D_2E2F,
                    0x3031_0000,
                ][..]
            )
            .unwrap()
            .payload()
            .collect::<std::vec::Vec<u8>>(),
            std::vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29,
                0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31
            ]
        );
    }

    #[test]
    fn payload_nth() {
        let message = Sysex8::try_from(
            &[
                0x541E_BB00,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x542E_BB0D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x542E_BB1A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x543C_BB27,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ][..],
        )
        .unwrap();
        let mut payload = message.payload();
        assert_eq!(payload.len(), 50);
        assert_eq!(payload.nth(13), Some(0x0D));
        assert_eq!(payload.len(), 36);
        assert_eq!(payload.nth(11), Some(0x19));
        assert_eq!(payload.len(), 24);
        assert_eq!(payload.nth(11), Some(0x25));
        assert_eq!(payload.len(), 12);
        assert_eq!(payload.nth(4), Some(0x2A));
        assert_eq!(payload.len(), 7);
        assert_eq!(payload.nth(5), Some(0x30));
        assert_eq!(payload.len(), 1);
        assert_eq!(payload.next(), Some(0x31));
        assert_eq!(payload.len(), 0);
        assert_eq!(payload.next(), None);
    }

    #[test]
    fn payload_nth_non_contiguous_payload() {
        let message = Sysex8::try_from(
            &[
                // empty
                0x5411_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5422_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5423_BB01,
                0x0200_0000,
                0x0000_0000,
                0x0000_0000,
                // empty
                0x5421_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                // empty
                0x5421_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5424_BB03,
                0x0405_0000,
                0x0000_0000,
                0x0000_0000,
                0x5425_BB06,
                0x0708_0900,
                0x0000_0000,
                0x0000_0000,
                0x5426_BB0A,
                0x0B0C_0D0E,
                0x0000_0000,
                0x0000_0000,
                0x5427_BB0F,
                0x1011_1213,
                0x1400_0000,
                0x0000_0000,
                0x5428_BB15,
                0x1617_1819,
                0x1A1B_0000,
                0x0000_0000,
                0x5429_BB1C,
                0x1D1E_1F20,
                0x2122_2300,
                0x0000_0000,
                0x542A_BB24,
                0x2526_2728,
                0x292A_2B2C,
                0x0000_0000,
                0x5436_BB2D,
                0x2E2F_3031,
                0x0000_0000,
                0x0000_0000,
            ][..],
        )
        .unwrap();
        let mut payload = message.payload();
        assert_eq!(payload.len(), 50);
        assert_eq!(payload.nth(13), Some(0x0D));
        assert_eq!(payload.len(), 36);
        assert_eq!(payload.nth(11), Some(0x19));
        assert_eq!(payload.len(), 24);
        assert_eq!(payload.nth(11), Some(0x25));
        assert_eq!(payload.len(), 12);
        assert_eq!(payload.nth(4), Some(0x2A));
        assert_eq!(payload.len(), 7);
        assert_eq!(payload.nth(5), Some(0x30));
        assert_eq!(payload.len(), 1);
        assert_eq!(payload.next(), Some(0x31));
        assert_eq!(payload.len(), 0);
        assert_eq!(payload.next(), None);
    }

    #[test]
    fn payload_next_non_contiguous_payload() {
        let message = Sysex8::try_from(
            &[
                // empty
                0x5411_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5422_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5423_BB01,
                0x0200_0000,
                0x0000_0000,
                0x0000_0000,
                // empty
                0x5421_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                // empty
                0x5421_BB00,
                0x0000_0000,
                0x0000_0000,
                0x0000_0000,
                0x5424_BB03,
                0x0405_0000,
                0x0000_0000,
                0x0000_0000,
                0x5425_BB06,
                0x0708_0900,
                0x0000_0000,
                0x0000_0000,
                0x5426_BB0A,
                0x0B0C_0D0E,
                0x0000_0000,
                0x0000_0000,
                0x5427_BB0F,
                0x1011_1213,
                0x1400_0000,
                0x0000_0000,
                0x5428_BB15,
                0x1617_1819,
                0x1A1B_0000,
                0x0000_0000,
                0x5429_BB1C,
                0x1D1E_1F20,
                0x2122_2300,
                0x0000_0000,
                0x542A_BB24,
                0x2526_2728,
                0x292A_2B2C,
                0x0000_0000,
                0x5436_BB2D,
                0x2E2F_3031,
                0x0000_0000,
                0x0000_0000,
            ][..],
        )
        .unwrap();
        let mut payload = message.payload();
        for i in 0..50 {
            assert_eq!(payload.len(), 50 - i);
            assert_eq!(payload.next(), Some(i as u8));
        }
    }

    #[test]
    fn set_payload() {
        let mut message = Sysex8::<std::vec::Vec<u32>>::new();
        message.set_payload(0..20);
        assert_eq!(
            message,
            Sysex8(std::vec![
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5038_000D,
                0x0E0F_1011,
                0x1213_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn set_rubbish_payload() {
        use crate::detail::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        let mut message = Sysex8::<std::vec::Vec<u32>>::new();
        message.set_payload(RubbishPayloadIterator::new());
        assert_eq!(
            message,
            Sysex8(std::vec![
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x502E_000D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x502E_001A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x503C_0027,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ]),
        );
    }

    #[test]
    fn set_rubbish_payload_to_fixed_size_buffer() {
        use crate::detail::test_support::rubbish_payload_iterator::RubbishPayloadIterator;
        let mut message = Sysex8::<[u32; 16]>::new();
        assert_eq!(
            message.try_set_payload(RubbishPayloadIterator::new()),
            Ok(())
        );
        assert_eq!(
            message,
            Sysex8([
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x502E_000D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x502E_001A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x503C_0027,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ]),
        );
    }

    #[test]
    fn set_and_reset_payload_decreasing() {
        let mut message = Sysex8::<std::vec::Vec<u32>>::new();
        message.set_payload(0..30);
        message.set_payload(0..20);
        assert_eq!(
            message,
            Sysex8(std::vec![
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5038_000D,
                0x0E0F_1011,
                0x1213_0000,
                0x0000_0000,
            ]),
        );
    }

    #[test]
    fn set_and_reset_payload_fixed_size_buffer() {
        let mut message = Sysex8::<[u32; 13]>::new();
        assert_eq!(message.try_set_payload(0..30), Ok(()));
        assert_eq!(message.try_set_payload(0..20), Ok(()));
        assert_eq!(
            message.data(),
            &[
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x5038_000D,
                0x0E0F_1011,
                0x1213_0000,
                0x0000_0000,
            ],
        );
    }

    #[test]
    fn set_payload_to_fixed_size_buffer_with_overflow() {
        let mut message = Sysex8::<[u32; 16]>::new();
        assert_eq!(
            message.try_set_payload(0..60),
            Err(crate::error::BufferOverflow)
        );
    }

    #[test]
    fn default_constructed_message() {
        assert_eq!(
            Sysex8::<std::vec::Vec<u32>>::new(),
            Sysex8(std::vec![0x5001_0000, 0x0, 0x0, 0x0,])
        );
    }

    #[test]
    fn payload_of_empty_message() {
        let message = Sysex8::<std::vec::Vec<u32>>::new();
        let payload = message.payload().collect::<std::vec::Vec<u8>>();
        assert_eq!(payload, std::vec::Vec::<u8>::new());
    }

    #[test]
    fn packets() {
        use crate::Packets;

        let message = Sysex8::try_from(
            &[
                0x501E_0000,
                0x0102_0304,
                0x0506_0708,
                0x090A_0B0C,
                0x502E_000D,
                0x0E0F_1011,
                0x1213_1415,
                0x1617_1819,
                0x502E_001A,
                0x1B1C_1D1E,
                0x1F20_2122,
                0x2324_2526,
                0x503C_0027,
                0x2829_2A2B,
                0x2C2D_2E2F,
                0x3031_0000,
            ][..],
        )
        .unwrap();

        let mut packets = message.packets();
        assert_eq!(
            &*packets.next().unwrap(),
            &[0x501E_0000, 0x0102_0304, 0x0506_0708, 0x090A_0B0C,][..]
        );
        assert_eq!(
            &*packets.next().unwrap(),
            &[0x502E_000D, 0x0E0F_1011, 0x1213_1415, 0x1617_1819,][..]
        );
        assert_eq!(
            &*packets.next().unwrap(),
            &[0x502E_001A, 0x1B1C_1D1E, 0x1F20_2122, 0x2324_2526,][..]
        );
        assert_eq!(
            &*packets.next().unwrap(),
            &[0x503C_0027, 0x2829_2A2B, 0x2C2D_2E2F, 0x3031_0000,][..]
        );
        assert_eq!(packets.next(), None);
    }

    #[test]
    fn packets_empty() {
        use crate::Packets;

        let message = Sysex8::<[u32; 4]>::new();
        let mut packets = message.packets();
        assert_eq!(&*packets.next().unwrap(), &[0x5001_0000, 0x0, 0x0, 0x0][..]);
        assert_eq!(packets.next(), None);
    }
}
