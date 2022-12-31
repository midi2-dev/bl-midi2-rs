#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SliceData<T, const N: usize>([T; N], usize)
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default;

impl<T, const N: usize> SliceData<T, N>
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default,
{
    pub const LEN: usize = N;

    pub fn resize(&mut self, sz: usize) {
        assert!(sz <= N);
        self.1 = sz;
    }

    pub fn from_data(d: &[T]) -> Self {
        assert!(d.len() <= N);
        let mut ret: Self = SliceData::<T, N> {
            1: d.len(),
            ..Default::default()
        };
        ret.0[0..d.len()].clone_from_slice(d);
        ret
    }

    pub fn push(&mut self, value: T) {
        self.resize(self.1 + 1);
        self.0[self.1 - 1] = value;
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(
            <SliceData::<u8, 5> as Default>::default(),
            SliceData::<u8, 5>([0, 0, 0, 0, 0], 0),
        );
    }

    #[test]
    fn resize() {
        let mut slice_data: SliceData<u8, 5> = Default::default();
        slice_data.resize(3);
        assert_eq!(slice_data.len(), 3);
    }

    #[test]
    fn from_data() {
        assert_eq!(&*SliceData::<u8, 8>::from_data(&[1, 2, 3]), &[1, 2, 3],);
    }
}
