use anyhow::{Error, Result};
use aoc2022::read_input;

fn main() -> Result<()> {
    let input = read_input!()?;
    let packet = parse_input(&input)?;
    println!("{}", start_pos(packet, 4)?);
    println!("{}", start_pos(packet, 14)?);
    Ok(())
}

fn start_pos(packet: &[u8], len: usize) -> Result<usize> {
    let mut i = 0;
    'outer: while i < packet.len() - len {
        let mut set = 0u32;
        for j in (i..i + len).rev() {
            let mask = 1 << (packet[j] - b'a');
            if set & mask != 0 {
                i = j + 1;
                continue 'outer;
            }
            set |= mask;
        }
        return Ok(i + len);
    }
    Err(Error::msg("no start marker found"))
}

fn parse_input(input: &str) -> Result<&[u8]> {
    let packet = input.trim().as_bytes();
    if packet.iter().all(u8::is_ascii_lowercase) {
        Ok(packet)
    } else {
        Err(Error::msg("invalid character in input"))
    }
}
