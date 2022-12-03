use std::collections::HashSet;

use anyhow::{Error, Result};
use aoc2022::read_input;

fn main() -> Result<()> {
    let input = read_input!()?;
    let rucksacks: Vec<&str> = input.lines().collect();
    let common_item_priority = rucksacks
        .iter()
        .flat_map(common_item)
        .map(priority)
        .sum::<Result<u32>>()?;
    println!("{}", common_item_priority);
    let badge_priority = rucksacks
        .chunks(3)
        .flat_map(badge)
        .map(priority)
        .sum::<Result<u32>>()?;
    println!("{}", badge_priority);
    Ok(())
}

fn priority(item: char) -> Result<u32> {
    match item {
        'a'..='z' => Ok(item as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(item as u32 - 'A' as u32 + 27),
        _ => Err(Error::msg("invalid item")),
    }
}

fn common_item(rucksack: &&str) -> Option<char> {
    let mut contents = rucksack.chars();
    let first = HashSet::<_>::from_iter(contents.by_ref().take(rucksack.len() / 2));
    contents.find(|item| first.contains(item))
}

fn badge(rucksacks: &[&str]) -> Option<char> {
    rucksacks
        .iter()
        .map(|r| HashSet::<_>::from_iter(r.chars()))
        .fold(None, |acc, items| match acc {
            None => Some(items),
            Some(acc) => Some(&acc & &items),
        })
        .into_iter()
        .flatten()
        .next()
}
