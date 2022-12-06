use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc2022::read_input;

fn main() -> Result<()> {
    let input = read_input!()?;
    let sop = start_marker(input.as_bytes(), 4).context("no start of packet found")?;
    println!("{sop}");
    let som = start_marker(input.as_bytes(), 14).context("no start of message found")?;
    println!("{som}");
    Ok(())
}

fn start_marker(packet: &[u8], len: usize) -> Option<usize> {
    packet
        .windows(len)
        .position(|win| HashSet::<_>::from_iter(win).len() == len)
        .map(|i| i + len)
}
