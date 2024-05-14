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

pub trait Packets {
    fn packets(&self) -> PacketsIterator;
}
