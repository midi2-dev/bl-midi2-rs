macro_rules! simple_generic_message {
    ($op_code:expr, $name:ident, $builder_name:ident) => {
        use crate::{
            message::{
                helpers as message_helpers,
                system_common::{self, TYPE_CODE as SYSTEM_COMMON_TYPE_CODE},
            },
            result::Result,
            *,
        };

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name<'a, B: Buffer>(&'a B::Data);


        impl<'a> Message<'a, Ump> for $name<'a, Ump> {
            fn data(&self) -> &'a [u32] {
                self.0
            }
            fn from_data_unchecked(data: &'a [u32]) -> Self {
                Self(data)
            }
            fn validate_data(data: &[u32]) -> Result<()> {
                system_common::validate_packet(data, $op_code)?;
                Ok(())
            }
        }

        impl<'a> Buildable<'a, Ump> for $name<'a, Ump> {
            type Builder = $builder_name<'a, Ump>;
        }

        impl<'a> Buildable<'a, Bytes> for $name<'a, Bytes> {
            type Builder = $builder_name<'a, Bytes>;
        }

        impl<'a> Message<'a, Bytes> for $name<'a, Bytes> {
            fn data(&self) -> &'a [u8] {
                self.0
            }
            fn from_data_unchecked(data: &'a [u8]) -> Self {
                Self(data)
            }
            fn validate_data(data: &[u8]) -> Result<()> {
                system_common::validate_bytes(data, $op_code, 1)?;
                Ok(())
            }
        }

        impl<'a> GroupedMessage<'a> for $name<'a, Ump> {
            fn group(&self) -> u4 {
                message_helpers::group_from_packet(self.0)
            }
        }

        #[derive(PartialEq, Eq)]
        pub struct $builder_name<'a, B: Buffer>(Result<&'a mut B::Data>);

        impl<'a> Builder<'a, Ump> for $builder_name<'a, Ump> {
            type Message = $name<'a, Ump>;
            fn build(self) -> Result<$name<'a, Ump>> {
                match self.0 {
                    Ok(buffer) => Ok($name(buffer)),
                    Err(e) => Err(e.clone()),
                }
            }
            fn new(buffer: &'a mut [u32]) -> Self {
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
        }

        impl<'a> Builder<'a, Bytes> for $builder_name<'a, Bytes> {
            type Message = $name<'a, Bytes>;
            fn new(buffer: &'a mut [u8]) -> Self {
                if buffer.len() >= 1 {
                    message_helpers::clear_buffer(buffer);
                    buffer[0] = $op_code;
                    Self(Ok(&mut buffer[..1]))
                } else {
                    Self(Err(Error::BufferOverflow))
                }
            }
            fn build(self) -> Result<Self::Message> {
                match self.0 {
                    Ok(buffer) => Ok($name(buffer)),
                    Err(e) => Err(e.clone()),
                }
            }
        }

        impl<'a> GroupedBuilder<'a> for $builder_name<'a, Ump> {
            fn group(mut self, v: u4) -> Self {
                if let Ok(buffer) = &mut self.0 {
                    message_helpers::write_group_to_packet(v, buffer);
                }
                self
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
    use crate::util::RandomBuffer;

    simple_generic_message!(0xFF, TestMessage, TestBuilder);

    #[test]
    fn builder() {
        assert_eq!(
            TestMessage::<Ump>::builder(&mut Ump::random_buffer::<1>())
                .group(u4::new(0x9))
                .build(),
            Ok(TestMessage::<Ump>(&[0x19FF_0000])),
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            TestMessage::<Ump>::from_data(&[0x19FF_0000])
                .unwrap()
                .group(),
            u4::new(0x9),
        );
    }

    #[test]
    fn bytes_builder() {
        assert_eq!(
            TestMessage::<Bytes>::builder(&mut Bytes::random_buffer::<3>()).build(),
            Ok(TestMessage::<Bytes>(&[0xFF])),
        );
    }

    #[test]
    fn bytes_from_data() {
        assert_eq!(
            TestMessage::<Bytes>::from_data(&[0xFF]),
            Ok(TestMessage::<Bytes>(&[0xFF])),
        )
    }
}
