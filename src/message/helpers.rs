use crate::{error::Error, result::Result, util::BitOps, *};

pub fn write_type_to_packet(t: u4, p: &mut [u32]) {
    p[0].set_nibble(0, t);
}

pub fn group_from_packet(p: &[u32]) -> u4 {
    p[0].nibble(1)
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

pub fn check_flex_data_or_ump_stream_consistent_packet_formats(
    buffer: &[u32],
    format_crumb_index: usize,
) -> Result<()> {
    // complete message
    if buffer.len() == 4 && buffer[0].crumb(format_crumb_index) != u2::new(0b00) {
        return Err(Error::InvalidData);
    } else if buffer.len() > 4 {
        // composite message
        let mut packets = buffer.chunks_exact(4).peekable();
        // start
        if packets.next().unwrap()[0].crumb(format_crumb_index) != u2::new(0b01) {
            return Err(Error::InvalidData);
        }

        while let Some(packet) = packets.next() {
            if packets.peek().is_some() {
                // continue
                if packet[0].crumb(format_crumb_index) != u2::new(0b10) {
                    return Err(Error::InvalidData);
                }
            } else {
                // end
                if packet[0].crumb(format_crumb_index) != u2::new(0b11) {
                    return Err(Error::InvalidData);
                }
            }
        }
    }

    Ok(())
}

// assumes that buffer contains valid messages
pub fn validate_sysex_group_statuses<
    IsComplete: Fn(u4) -> bool,
    IsBegin: Fn(u4) -> bool,
    IsContinue: Fn(u4) -> bool,
    IsEnd: Fn(u4) -> bool,
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
