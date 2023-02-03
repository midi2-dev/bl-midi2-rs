#[derive(Debug)]
pub enum Error {
    InvalidData,
    BufferOverflow,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Impl: core::ops::DerefMut<Target = [u32]> {
    fn resize(&mut self, size: usize) -> Result<()>;
}

pub trait Message {
    fn data(&self) -> &[u32];
    fn validate(data: &[u32]) -> Result<()>;
}

pub trait Borrowed<'a> {
    #[cfg(feature = "std")]
    type Owned;
    type OwnedFixedSize<const SIZE: usize>;
    #[cfg(feature = "std")]
    fn into_owned(self) -> Self::Owned;
    fn into_owned_fixed_size<const SIZE: usize>(self, buff: [u32; SIZE]) -> Result<Self::OwnedFixedSize<SIZE>>;
}

mod statically_sized {
    use core::ops::{Deref, DerefMut};

    pub struct Impl<const SIZE: usize>([u32; SIZE], usize);
    
    impl<const SIZE: usize> Impl<SIZE> {
        pub fn new(data: [u32; SIZE]) -> Self {
            Impl(data, 0)
        }
    }

    impl<const SIZE: usize> core::ops::Deref for Impl<SIZE> {
        type Target = [u32];
        fn deref(&self) -> &[u32] {
            &self.0[0..self.1]
        }
    }

    impl<const SIZE: usize> core::ops::DerefMut for Impl<SIZE> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0[0..self.1]
        }
    }

    impl<const SIZE: usize> core::convert::AsRef<[u32]> for Impl<SIZE> {
        fn as_ref(&self) -> &[u32] {
            self.deref()
        }
    }

    impl<const SIZE: usize> core::convert::AsMut<[u32]> for Impl<SIZE> {
        fn as_mut(&mut self) -> &mut [u32] {
            self.deref_mut()
        }
    }

    impl<const SIZE: usize> super::Impl for Impl<SIZE> {
        fn resize(&mut self, size: usize) -> super::Result<()> {
            if size > SIZE {
                Err(super::Error::BufferOverflow{})                
            } else {
                self.1 = size;
                Ok(())
            }
        }
    }
}

mod borrowed {
    use core::ops::Deref;
    pub struct Impl<'a>(&'a [u32]);
    
    impl<'a> Impl<'a> {
        pub fn new(data: &'a [u32]) -> Self {
            Impl(data)
        }
    }

    impl core::ops::Deref for Impl<'_> {
        type Target = [u32];
        fn deref(&self) -> &Self::Target {
            self.0
        }
    }
     
    impl core::ops::DerefMut for Impl<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unreachable!()
        }
    }

    impl core::convert::AsRef<[u32]> for Impl<'_> {
        fn as_ref(&self) -> &[u32] {
            self.deref()
        }
    }
    
    impl super::Impl for Impl<'_> {
        fn resize(&mut self, size: usize) -> super::Result<()> {
            if size != self.0.len() {
                Err(super::Error::BufferOverflow{})
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(feature = "std")]
mod dynamically_sized {
    use core::ops::{Deref, DerefMut};
    pub struct Impl(std::vec::Vec<u32>);
    
    impl Impl {
        pub fn new() -> Self {
            Impl(std::vec::Vec::new())
        }
    }
    
    impl core::ops::Deref for Impl {
        type Target = [u32];
        fn deref(&self) -> &[u32] {
            &self.0
        }
    }
    
    impl core::ops::DerefMut for Impl {
        fn deref_mut(&mut self) -> &mut [u32] {
            &mut self.0
        }
    }
    
    impl core::convert::AsRef<[u32]> for Impl {
        fn as_ref(&self) -> &[u32] {
            self.deref()
        }
    }

    impl core::convert::AsMut<[u32]> for Impl {
        fn as_mut(&mut self) -> &mut [u32] {
            self.deref_mut()
        }
    }
    
    impl super::Impl for Impl {
        fn resize(&mut self, size: usize) -> super::Result<()> {
            self.0.resize(size, 0);
            Ok(())
        }
    }
}


#[cfg(test)]
mod test_message {
    use crate::util::BitOps;
    use super::{Impl, Message as _, Borrowed as _};

    pub struct SpecializedImpl<I: super::Impl>(I);
    
    impl<I: super::Impl> SpecializedImpl<I> {
        fn borrow<'a>(data: &'a [u32]) -> super::Result<SpecializedImpl<super::borrowed::Impl<'a>>> {
            todo!()
        }
        #[cfg(feature = "std")]
        fn default() -> SpecializedImpl<super::dynamically_sized::Impl> {
            todo!()
        }
        fn default_fixed_size<const SIZE: usize>(buff: [u32; SIZE]) -> super::Result<SpecializedImpl<super::statically_sized::Impl<SIZE>>> {
            todo!()
        }
        fn group(&self) -> ux::u4 {
            self.0[0].nibble(1)
        }
        fn set_group(&mut self, g: ux::u4) -> &mut Self {
            self.0[0].set_nibble(1, g);
            self
        }
    }

    type Specialized<'a> = SpecializedImpl<super::borrowed::Impl<'a>>;
    
    impl<I: super::Impl> super::Message for SpecializedImpl<I> {
        fn data(&self) -> &[u32] {
            self.0.deref()
        }
        fn validate(_data: &[u32]) -> super::Result<()> {
            Ok(())
        }
    }
    
    impl<'a> super::Borrowed<'a> for SpecializedImpl<super::borrowed::Impl<'a>> {
        type Owned = SpecializedImpl<super::dynamically_sized::Impl>;
        type OwnedFixedSize<const SIZE: usize> = SpecializedImpl<super::statically_sized::Impl<SIZE>>;
        #[cfg(feature = "std")]
        fn into_owned(self) -> Self::Owned {
            let mut i = super::dynamically_sized::Impl::new();
            i.resize(self.data().len()).unwrap();
            i.copy_from_slice(self.data());
            SpecializedImpl(i)
        }
        fn into_owned_fixed_size<const SIZE: usize>(self, buff: [u32; SIZE]) -> super::Result<Self::OwnedFixedSize<SIZE>> {
            let mut i = super::statically_sized::Impl::new(buff);
            i.resize(self.data().len())?;
            i.copy_from_slice(self.data());
            Ok(SpecializedImpl(i))
        }
    }
    
    #[test]
    #[cfg(feature = "std")]
    fn new_owned() {
        assert_eq!(Specialized::default().group(), ux::u4::new(0x0));
        let _ = Specialized::default_fixed_size([]).unwrap().set_group(ux::u4::new(0xA));
        let _ = Specialized::borrow(&[0x0]).unwrap().set_group(ux::u4::new(0xB));
    }
}