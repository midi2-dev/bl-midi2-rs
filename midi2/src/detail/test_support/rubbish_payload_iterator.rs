/// This is for testing payload insertion implementation on sysex messages.
/// The iterator returns no size hints so the optimisation case for these
/// payload insertion implementations will hit their worst case for mem-copying.
pub struct RubbishPayloadIterator<I: core::iter::Iterator<Item = u8>>(I);

const DEFAULT_DATA: [u8; 50] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
];

impl RubbishPayloadIterator<core::iter::Cloned<core::slice::Iter<'static, u8>>> {
    pub fn new() -> Self {
        RubbishPayloadIterator(DEFAULT_DATA.iter().cloned())
    }
}

impl<I: core::iter::Iterator<Item = u8>> core::convert::From<I> for RubbishPayloadIterator<I> {
    fn from(iter: I) -> Self {
        RubbishPayloadIterator(iter)
    }
}

impl<I: core::iter::Iterator<Item = u8>> core::iter::Iterator for RubbishPayloadIterator<I> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

mod tests {
    use super::*;

    #[test]
    fn rubbish_iterator_should_give_worst_case_bounds() {
        assert_eq!(RubbishPayloadIterator::new().size_hint(), (0, None));
    }
}
