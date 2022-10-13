use crate::{
    error::Error,
    packet::Packet,
};

struct Message {
    channel: ux::u4,
    note: ux::u7,
    detach: bool,
    reset: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
}