use crate::{
    packet::Packet,
};

pub trait Message: 
    Clone + 
    core::fmt::Debug + 
    Default + 
    PartialEq + 
    core::convert::TryFrom<Packet> +
    core::convert::Into<Packet>
{
    fn group(&self) -> ux::u4;
    fn set_group(self, group: ux::u4) -> Self;
}

macro_rules! message_trait_impl {
    ($t:ident) => {
        impl message_trait::Message for $t {
            fn group(&self) -> ux::u4 {
                self.group
            }

             fn set_group(self, group: ux::u4) -> Self {
                 Self {
                     group,
                     ..self
                 }
             }
        }
    }
}

pub(crate) use message_trait_impl;

