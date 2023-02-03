macro_rules! message_debug_impl {
    ($type_name:ident) => {
        impl core::fmt::Debug for $type_name<'_> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_fmt(format_args!("{}(", stringify!($type_name)))?;
                for v in self.0.iter() {
                    fmt.write_fmt(format_args!("{v:#010X}, "))?;
                }
                fmt.write_str(")")
            }
        }
    }
}

pub(crate) use message_debug_impl;