#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SliceData<T, const N: usize>([T; N], usize)
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default;

impl<T, const N: usize> core::ops::Deref for SliceData<T, N>
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0[0..self.1]
    }
}

impl<T, const N: usize> core::ops::DerefMut for SliceData<T, N>
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[0..self.1]
    }
}
