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
    }
}

pub(crate) use message_debug_impl;