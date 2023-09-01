macro_rules! simple_generic_message {
    ($op_code:expr, $name:ident, $builder_name:ident) => {
        use crate::{
            message::{
                helpers as message_helpers,
                system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
            },
            result::Result,
            util::debug,
            *,
        };

        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<'a>(&'a [u32]);

        debug::message_debug_impl!($name);

        impl<'a> $name<'a> {
            pub fn builder(buffer: &mut [u32]) -> $builder_name {
                $builder_name::new(buffer)
            }
            pub fn group(&self) -> u4 {
                message_helpers::group_from_packet(self.0)
            }
            pub fn from_data(data: &'a [u32]) -> Result<Self> {
                match system_common::validate_packet(data, $op_code) {
                    Err(e) => Err(e),
                    Ok(()) => Ok(Self(data)),
                }
            }
        }

        #[derive(PartialEq, Eq)]
        pub struct $builder_name<'a>(Result<&'a mut [u32]>);

        impl<'a> $builder_name<'a> {
            pub fn new(buffer: &'a mut [u32]) -> Self {
                match system_common::validate_buffer_size(buffer) {
                    Ok(()) => {
                        message_helpers::clear_buffer(buffer);
                        system_common::write_op_code_to_packet(buffer, $op_code);
                        message_helpers::write_type_to_packet(SYSTEM_COMMON_TYPE_CODE, buffer);
                        Self(Ok(buffer))
                    }
                    Err(e) => Self(Err(e)),
                }
            }
            pub fn group(&mut self, v: u4) -> &mut Self {
                if let Ok(buffer) = &mut self.0 {
                    message_helpers::write_group_to_packet(v, buffer);
                }
                self
            }
            pub fn build(&'a self) -> Result<$name<'a>> {
                match &self.0 {
                    Ok(buffer) => Ok($name(buffer)),
                    Err(e) => Err(e.clone()),
                }
            }
        }
    };
}

pub(crate) use simple_generic_message;

pub mod tune_request {
    use super::simple_generic_message;
    simple_generic_message!(0xF6, TuneRequestMessage, TuneRequestBuilder);
}
pub mod timing_clock {
    use super::simple_generic_message;
    simple_generic_message!(0xF8, TimingClockMessage, TimingClockBuilder);
}
pub mod start {
    use super::simple_generic_message;
    simple_generic_message!(0xFA, StartMessage, StartBuilder);
}
pub mod cont {
    use super::simple_generic_message;
    simple_generic_message!(0xFB, ContinueMessage, ContinueBuilder);
}
pub mod stop {
    use super::simple_generic_message;
    simple_generic_message!(0xFC, StopMessage, StopBuilder);
}
pub mod active_sensing {
    use super::simple_generic_message;
    simple_generic_message!(0xFE, ActiveSensingMessage, ActiveSensingBuilder);
}
pub mod reset {
    use super::simple_generic_message;
    simple_generic_message!(0xFF, ResetMessage, ResetBuilder);
}

#[cfg(test)]
mod tests {
    use super::simple_generic_message;
    use crate::util::random_buffer;

    simple_generic_message!(0xFF, TestMessage, TestBuilder);

    #[test]
    fn builder() {
        assert_eq!(
            TestMessage::builder(&mut random_buffer::<1>()).group(u4::new(0x9)).build(),
            Ok(TestMessage(&[0x19FF_0000])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TestMessage::from_data(&[0x19FF_0000]).unwrap().group(),
            u4::new(0x9),
        );
    }
}
