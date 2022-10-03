use crate::error::Error;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SliceData<T, const N: usize>(pub [T; N], pub usize)
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default;

impl<T, const N: usize> SliceData<T, N>
where
    T: Clone + core::fmt::Debug + Default + PartialEq,
    [T; N]: Default
{
    pub fn data(&self) -> &[T] {
        &self.0[0..self.1]
    }

    pub fn set_data(&mut self, d: &[T]) -> Result<&mut SliceData::<T, N>, Error> {
        if d.len() >= N {
            Err(Error::BufferOverflow)
        } else {
            self.0[0..d.len()].clone_from_slice(&d);
            self.1 = d.len();
            Ok(self)
        }
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
    fn data() {
        assert_eq!(
            SliceData::<u8, 5>([1, 2, 3, 0, 0], 3).data(),
            &[1, 2, 3],
        );
            
    }

    #[test]
    fn set_data() {
        assert_eq!(
            <SliceData::<u8, 5> as Default>::default().set_data(&[1, 2]),
            Ok(&mut SliceData::<u8, 5>([1, 2, 0, 0, 0], 2)),
        );
    }
}
