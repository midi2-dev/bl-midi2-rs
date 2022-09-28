pub trait Message: Clone + core::fmt::Debug + Default + PartialEq {
    fn group(&self) -> ux::u4;
    fn set_group(self, group: ux::u4) -> Self;
}
