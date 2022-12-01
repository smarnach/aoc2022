use anyhow::Result;
use aoc2022::{parse_lines, read_input};

fn main() -> Result<()> {
    let input = read_input!()?;
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|elf| parse_lines::<u32>(elf).sum::<Result<u32>>())
        .collect::<Result<_>>()?;
    calories.sort_unstable();
    println!("{}", calories.last().unwrap());
    println!("{}", calories[calories.len() - 3..].iter().sum::<u32>());
    Ok(())
}
