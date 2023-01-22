use crate::error::InvalidData;
use core::ops::Deref;

pub trait MessagePrivate<'a, const SIZE: usize> {
    type Owned<'b: 'a>: Message<'b, SIZE>;
    fn new(message_impl: MessageImpl<'a, SIZE>) -> Self;
    fn message_impl(&self) -> &MessageImpl<'a, SIZE>;
    fn message_impl_mut(&mut self) -> &mut MessageImpl<'a, SIZE>;
    fn default_data() -> [u32; SIZE];
}

pub trait Message<'a, const SIZE: usize>
where
    Self: Sized + MessagePrivate<'a, SIZE>,
{
    fn default() -> Self {
        Self::new(MessageImpl::from_owned(Self::default_data()))
    }
    fn validate(data: &[u32]) -> Result<(), InvalidData>;
    fn try_new_borrowed<'b: 'a>(
        data: &'a [u32],
    ) -> Result<BorrowedMessage<'a, 'b, SIZE, Self, Self::Owned<'b>>, InvalidData> {
        Self::validate(data)?;
        Ok(BorrowedMessage(
            data,
            Self::new(MessageImpl::from_borrowed(data)),
            Default::default(),
        ))
    }
    fn try_new_owned(data: &[u32]) -> Result<Self, InvalidData> {
        Self::validate(data)?;
        let mut owned_data = [0_u32; SIZE];
        owned_data[0..data.len()].copy_from_slice(data);
        Ok(Self::new(MessageImpl::from_owned(owned_data)))
    }
    fn data<'b: 'a>(&'b self) -> &'a [u32] {
        self.message_impl().deref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BorrowedMessage<
    'a,
    'b: 'a,
    const SIZE: usize,
    M: Message<'a, SIZE>,
    O: Message<'b, SIZE>,
>(&'a [u32], M, core::marker::PhantomData<&'b O>);

impl<'a, 'b: 'a, const SIZE: usize, M: Message<'a, SIZE>, O: Message<'b, SIZE>>
    BorrowedMessage<'a, 'b, SIZE, M, O>
where
    'b: 'a,
{
    pub fn into_owned(self) -> O {
        let mut owned_data = [0_u32; SIZE];
        owned_data[0..self.0.len()].copy_from_slice(self.0);
        O::new(MessageImpl::from_owned(owned_data))
    }
}

impl<'a, 'b, const SIZE: usize, M: Message<'a, SIZE>, O: Message<'b, SIZE>> core::ops::Deref
    for BorrowedMessage<'a, 'b, SIZE, M, O>
{
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum MessageImpl<'a, const SIZE: usize> {
    Owned([u32; SIZE]),
    Borrowed(&'a [u32]),
}

impl<'a, const SIZE: usize> MessageImpl<'a, SIZE> {
    pub fn from_borrowed<'b: 'a>(data: &'b [u32]) -> Self {
        MessageImpl::Borrowed(data)
    }
    pub fn from_owned(data: [u32; SIZE]) -> Self {
        MessageImpl::Owned(data)
    }
}

impl<'a, const SIZE: usize> core::ops::Deref for MessageImpl<'a, SIZE> {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        match self {
            MessageImpl::Owned(d) => d,
            MessageImpl::Borrowed(d) => d,
        }
    }
}

impl<'a, const SIZE: usize> core::ops::DerefMut for MessageImpl<'a, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MessageImpl::Owned(d) => d,
            MessageImpl::Borrowed(_) => unreachable!(),
        }
    }
}

impl<'a, const SIZE: usize> core::fmt::Debug for MessageImpl<'a, SIZE> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MessageImpl::Owned(_) => {
                fmt.write_str("Owned(")?;
            }
            MessageImpl::Borrowed(_) => {
                fmt.write_str("Borrowed(")?;
            }
        }
        for v in self.deref() {
            fmt.write_fmt(format_args!("{v:#010X}, "))?;
        }
        fmt.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use crate::util::BitOps;

    #[derive(Debug, PartialEq, Eq)]
    struct Specialised<'a>(super::MessageImpl<'a, 4>);

    impl<'a> super::MessagePrivate<'a, 4> for Specialised<'a> {
        type Owned<'b: 'a> = Specialised<'b>;
        fn new(message_impl: super::MessageImpl<'a, 4>) -> Self {
            Specialised(message_impl)
        }
        fn message_impl(&self) -> &super::MessageImpl<'a, 4> {
            &self.0
        }
        fn message_impl_mut(&mut self) -> &mut super::MessageImpl<'a, 4> {
            &mut self.0
        }
        fn default_data() -> [u32; 4] {
            [0; 4]
        }
    }

    impl<'a> super::Message<'a, 4> for Specialised<'a> {
        fn validate(_data: &[u32]) -> Result<(), super::InvalidData> {
            Ok(())
        }
    }

    impl Specialised<'_> {
        pub fn group(&self) -> ux::u4 {
            self.0[0].nibble(1)
        }

        pub fn set_group(&mut self, group: ux::u4) -> &mut Self {
            self.0[0].set_nibble(1, group);
            self
        }
    }

    #[test]
    fn new_borrowed() {
        let data: [u32; 3] = [1, 2, 3];
        let _ = Specialised::try_new_borrowed(&data);
    }

    #[test]
    fn borrowed_data() {
        assert_eq!(
            Specialised::try_new_borrowed(&[1, 2, 3]).unwrap().data(),
            &[1, 2, 3],
        );
    }

    #[test]
    fn new_owned() {
        assert_eq!(
            Specialised::try_new_owned(&[1, 2, 3]),
            Ok(Specialised(super::MessageImpl::Owned([1, 2, 3, 0]))),
        );
    }

    #[test]
    fn owned_scoping() {
        let owned = {
            let buffer: [u32; 3] = [1, 2, 3];
            Specialised::try_new_owned(&buffer)
        };
        assert_eq!(
            owned,
            Ok(Specialised(super::MessageImpl::Owned([1, 2, 3, 0]))),
        );
    }

    #[test]
    fn borrowed_into_owned_scoping() {
        let owned = {
            let buffer: [u32; 3] = [1, 2, 3];
            Specialised::try_new_borrowed(&buffer).unwrap().into_owned()
        };
        assert_eq!(owned, Specialised(super::MessageImpl::Owned([1, 2, 3, 0])),);
    }

    #[test]
    fn deref_specialised_message_from_borrowed() {
        assert_eq!(
            Specialised::try_new_borrowed(&[0x0A00_0000])
                .unwrap()
                .group(),
            ux::u4::new(0xA),
        );
    }

    #[test]
    fn deref_specialised_message_from_owned() {
        assert_eq!(
            Specialised::try_new_owned(&[0x0A00_0000]).unwrap().group(),
            ux::u4::new(0xA),
        );
    }

    #[test]
    fn mutate_specialised_message_from_owned() {
        assert_eq!(
            Specialised::default().set_group(ux::u4::new(0xA)),
            &mut Specialised(super::MessageImpl::Owned([0x0A00_0000, 0x0, 0x0, 0x0])),
        );
    }
}
