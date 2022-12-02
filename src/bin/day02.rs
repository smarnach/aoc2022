use anyhow::{Context, Error, Result};
use aoc2022::read_input;

fn main() -> Result<()> {
    let input = read_input!()?;
    let rounds = parse(&input)?;
    println!("{}", total_score(&rounds, strategy1)?);
    println!("{}", total_score(&rounds, strategy2)?);
    Ok(())
}

fn total_score<F>(rounds: &[(char, char)], strategy: F) -> Result<i32>
where
    F: Fn(Shape, char) -> Result<Shape>,
{
    rounds
        .iter()
        .map(|&(c, d)| {
            let a = Shape::from_code(c, "ABC")?;
            let b = strategy(a, d)?;
            Ok(round_score(a, b))
        })
        .sum()
}

fn strategy1(_: Shape, d: char) -> Result<Shape> {
    Shape::from_code(d, "XYZ")
}

fn strategy2(a: Shape, d: char) -> Result<Shape> {
    let b = match d {
        'X' => Shape::try_from((a as i32 + 2) % 3).unwrap(),
        'Y' => a,
        'Z' => Shape::try_from((a as i32 + 1) % 3).unwrap(),
        _ => return Err(Error::msg("invalid strategy")),
    };
    Ok(b)
}

fn round_score(a: Shape, b: Shape) -> i32 {
    let score = match b as i32 - a as i32 {
        0 => 3,
        1 | -2 => 6,
        2 | -1 => 0,
        _ => unreachable!(),
    };
    score + b as i32 + 1
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_code(c: char, code: &str) -> Result<Shape> {
        let index = code.chars().position(|d| c == d).context("invalid code")?;
        Shape::try_from(index as i32)
    }
}

impl TryFrom<i32> for Shape {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Shape::Rock),
            1 => Ok(Shape::Paper),
            2 => Ok(Shape::Scissors),
            _ => Err(Error::msg("invalid index")),
        }
    }
}

fn parse(input: &str) -> Result<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            Ok((
                chars.next().context("line too short")?,
                chars.nth(1).context("line too short")?,
            ))
        })
        .collect()
}
