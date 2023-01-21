macro_rules! builder_setter {
    ($member:ident: $t:ty) => {
        pub fn $member(&mut self, v: $t) -> &mut Self {
            self.$member = Some(v);
            self
        }
    };
}

macro_rules! builder_method {
    () => {
        pub fn builder() -> Builder {
            Builder::default()
        }
    };
}

macro_rules! builder {
    ($($member:ident: $t:ty),*) => {
        use crate::util::builder as builder_internal;

        #[derive(Clone, Default)]
        pub struct Builder {
            $($member: Option<$t>),*
        }

        impl Builder {
            $(builder_internal::builder_setter!($member: $t);)*

            /// Creates a new message instance.
            /// Panics if any required fields are missing!
            pub fn build(&self) -> Message {
                Message {
                    $(
                        $member: self.$member.unwrap_or_else(|| panic!("Missing fields")),
                    )*
                }
            }
        }
    }
}

pub(crate) use builder;
pub(crate) use builder_setter;
pub(crate) use builder_method;

#[cfg(test)]
mod tests {
    use super::{builder, builder_method};

    #[derive(Debug, PartialEq, Eq)]
    pub struct Message {
        note: ux::u7,
        group: ux::u4,
    }

    builder!(note: ux::u7, group: ux::u4);
    
    impl Message {
        builder_method!();
    }

    #[test]
    fn call_build() {
        assert_eq!(
            Message::builder()
                .note(ux::u7::new(0x54))
                .group(ux::u4::new(0xA))
                .build(),
            Message {
                note: ux::u7::new(0x54),
                group: ux::u4::new(0xA),
            },
        )
    }

    #[test]
    #[should_panic]
    fn call_build_fail() {
        Message::builder().note(ux::u7::new(0x54)).build();
    }
}
