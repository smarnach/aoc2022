use anyhow::{Error, Result};
use aoc2022::read_input;
use std::ops::BitAnd;

fn main() -> Result<()> {
    let priorities = read_input!()?
        .trim()
        .chars()
        .map(priority)
        .collect::<Result<Vec<u8>>>()?;
    let rucksacks: Vec<&[u8]> = priorities.split(|&i| i == NEWLINE).collect();
    println!("{}", rucksacks.iter().map(common_item).sum::<u32>());
    println!("{}", rucksacks.chunks(3).map(badge).sum::<u32>());
    Ok(())
}

const NEWLINE: u8 = u8::MAX;

fn priority(item: char) -> Result<u8> {
    match item {
        'a'..='z' => Ok(item as u8 - b'a' + 1),
        'A'..='Z' => Ok(item as u8 - b'A' + 27),
        '\n' => Ok(NEWLINE),
        _ => Err(Error::msg("invalid item")),
    }
}

fn bitset(rucksack: &[u8]) -> u64 {
    rucksack.iter().fold(0, |acc, &i| acc | (1 << i))
}

fn common_item(rucksack: &&[u8]) -> u32 {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    (bitset(first) & bitset(second)).trailing_zeros()
}

fn badge(rucksacks: &[&[u8]]) -> u32 {
    rucksacks
        .iter()
        .map(|r| bitset(r))
        .fold(u64::MAX, BitAnd::bitand)
        .trailing_zeros()
}
