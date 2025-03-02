/// This is for testing payload insertion implementation on sysex messages.
/// The iterator returns no size hints so the optimisation case for these
/// payload insertion implementations will hit their worst case for mem-copying.
pub struct RubbishPayloadIterator(u8);

impl RubbishPayloadIterator {
    pub fn new() -> Self {
        RubbishPayloadIterator(0)
    }
}

impl core::iter::Iterator for RubbishPayloadIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 50 {
            return None;
        }
        let ret = Some(self.0);
        self.0 += 1;
        ret
    }
}
