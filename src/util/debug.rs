macro_rules! message_debug_impl {
    ($type_name:ident) => {
        impl core::fmt::Debug for $type_name<'_> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!($type_name)))?;
                let mut iter = self.0.iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#010X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(",")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    };
}

macro_rules! message_debug_impl_owned {
    ($type_name:ident) => {
        impl core::fmt::Debug for $type_name {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!($type_name)))?;
                let mut iter = self.0.iter().peekable();
                while let Some(v) = iter.next() {
                    fmt.write_fmt(format_args!("{v:#010X}"))?;
                    if iter.peek().is_some() {
                        fmt.write_str(",")?;
                    }
                }
                fmt.write_str(")")
            }
        }
    };
}

pub(crate) use message_debug_impl;
pub(crate) use message_debug_impl_owned;

pub fn packet_debug_fmt(data: &[u32], fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
    fmt.write_fmt(format_args!("{}(", stringify!($type_name)))?;
    let mut iter = data.iter().peekable();
    while let Some(v) = iter.next() {
        fmt.write_fmt(format_args!("{v:#010X}"))?;
        if iter.peek().is_some() {
            fmt.write_str(",")?;
        }
    }
    fmt.write_str(")")
}

// dev tool for hex printing of u32 buffers
// helps to debug on failed test output
#[derive(PartialEq, Eq)]
pub struct Data<'a>(pub &'a [u32]);
message_debug_impl!(Data);

#[derive(PartialEq, Eq)]
pub struct ByteData<'a>(pub &'a [u8]);

impl<'a> core::fmt::Debug for ByteData<'a> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.write_fmt(format_args!("ByteData("))?;
        let mut iter = self.0.iter().peekable();
        while let Some(v) = iter.next() {
            fmt.write_fmt(format_args!("{v:#04X}"))?;
            if iter.peek().is_some() {
                fmt.write_str(",")?;
            }
        }
        fmt.write_str(")")
    }
}
