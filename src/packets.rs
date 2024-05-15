/// Iterator type for reading the individual packets of a
/// [Ump](crate::buffer::Ump) backed message.
///
/// Returned from [Packets::packets].
#[derive(Debug, Clone)]
pub struct PacketsIterator<'a>(pub(crate) core::slice::ChunksExact<'a, u32>);

impl<'a> core::iter::Iterator for PacketsIterator<'a> {
    type Item = &'a [u32];
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> core::iter::FusedIterator for PacketsIterator<'a> {}

impl<'a> core::iter::ExactSizeIterator for PacketsIterator<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// Read the individual packets of a message represented with UMP packets.
///
/// ## Basic Usage
///
/// ```rust
/// use midi2::prelude::*;
///
/// let mut message = flex_data::ProjectName::<Vec<u32>>::new();
/// message.set_text("Shadows of the Forgotten Cathedral");
///
/// let mut packets = message.packets();
///
/// assert_eq!(packets.next(), Some(&[0xD0500101, 0x53686164, 0x6F777320, 0x6F662074][..]));
/// assert_eq!(packets.next(), Some(&[0xD0900101, 0x68652046, 0x6F72676F, 0x7474656E][..]));
/// assert_eq!(packets.next(), Some(&[0xD0D00101, 0x20436174, 0x68656472, 0x616C0000][..]));
/// assert_eq!(packets.next(), None);
/// ```
///
/// Packets may be shorter than 128 bytes for certain messages which are represented by shorter
/// packets.
///
/// ```rust
/// use midi2::prelude::*;
///
/// let mut message = sysex7::Sysex7::<Vec<u32>>::new();
/// message.set_payload((0..20).map(u7::new));
///
/// let mut packets = message.packets();
///
/// assert_eq!(packets.next(), Some(&[0x30160001, 0x2030405][..]));
/// assert_eq!(packets.next(), Some(&[0x30260607, 0x8090A0B][..]));
/// assert_eq!(packets.next(), Some(&[0x30260C0D, 0xE0F1011][..]));
/// assert_eq!(packets.next(), Some(&[0x30321213, 0x0][..]));
/// assert_eq!(packets.next(), None);
/// ```
pub trait Packets {
    fn packets(&self) -> PacketsIterator;
}
