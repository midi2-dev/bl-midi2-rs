use crate::{
    error::Error,
    result::Result,
    util::{BitOps, Truncate},
};

pub fn validate_packet(p: &[u32], type_code: ux::u4, op_code: ux::u4) -> Result<()> {
    if p.is_empty() || p[0].nibble(0) != type_code || p[0].nibble(2) != op_code {
        Err(Error::InvalidData)
    } else {
        Ok(())
    }
}

pub fn write_type_to_packet(t: ux::u4, p: &mut [u32]) {
    p[0].set_nibble(0, t);
}

pub fn write_group_to_packet(g: ux::u4, p: &mut [u32]) {
    p[0].set_nibble(1, g);
}

pub fn write_data(
    type_code: ux::u4,
    group: ux::u4,
    op_code: ux::u4,
    channel: ux::u4,
    p: &mut [u32],
) {
    write_type_to_packet(type_code, p);
    write_group_to_packet(group, p);
    p[0].set_nibble(2, op_code).set_nibble(3, channel);
}

pub fn group_from_packet(p: &[u32]) -> ux::u4 {
    p[0].nibble(1)
}

pub fn channel_from_packet(p: &[u32]) -> ux::u4 {
    p[0].nibble(3)
}

pub fn concatenate(lsb: ux::u7, msb: ux::u7) -> ux::u14 {
    (ux::u14::from(msb) << 7) | ux::u14::from(lsb)
}

pub fn most_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 >> 7).truncate()
}

pub fn least_significant_bit(word_14: ux::u14) -> ux::u7 {
    (word_14 & ux::u14::new(0b00_0000_0011_1111)).truncate()
}

pub fn sysex_group_consistent_groups(buffer: &[u32], stride: usize) -> Result<()> {
    use group_from_packet as gfp;
    if buffer
        .chunks_exact(stride)
        .all(|chunk| gfp(chunk) == gfp(buffer))
    {
        Ok(())
    } else {
        Err(Error::InvalidData)
    }
}

// assumes that buffer contains valid messages
pub fn validate_sysex_group_statuses<
    IsComplete: Fn(ux::u4) -> bool,
    IsBegin: Fn(ux::u4) -> bool,
    IsContinue: Fn(ux::u4) -> bool,
    IsEnd: Fn(ux::u4) -> bool,
>(
    buffer: &[u32],
    is_complete: IsComplete,
    is_begin: IsBegin,
    is_continue: IsContinue,
    is_end: IsEnd,
    stride: usize,
) -> Result<()> {
    let mut iter = buffer.chunks(stride).peekable();
    let first_status = iter.next().unwrap()[0].nibble(2);

    if iter.peek().is_none() {
        if is_complete(first_status) {
            return Ok(());
        } else {
            return Err(Error::InvalidData);
        }
    }

    if !is_begin(first_status) {
        return Err(Error::InvalidData);
    }

    while let Some(chunk) = iter.next() {
        let status = chunk[0].nibble(2);
        if (iter.peek().is_some() && !is_continue(status)) && !is_end(status) {
            return Err(Error::InvalidData);
        }
    }

    Ok(())
}
