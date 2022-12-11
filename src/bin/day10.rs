use anyhow::{Context, Error, Result};
use aoc2022::{parse_lines, read_input};
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read_input!()?;
    let instructions = parse_lines(&input).collect::<Result<Vec<Instruction>>>()?;
    println!("{}", total_signal(&instructions));
    let image = render(&instructions);
    for line in image.chunks_exact(40) {
        println!("{}", line.iter().collect::<String>());
    }
    Ok(())
}

fn total_signal(instructions: &[Instruction]) -> i32 {
    Cycles::new(instructions)
        .skip(19)
        .step_by(40)
        .map(|(cycle, x)| cycle as i32 * x)
        .sum()
}

fn render(instructions: &[Instruction]) -> Vec<char> {
    Cycles::new(instructions)
        .map(|(cycle, x)| {
            let column = (cycle - 1) % 40;
            if x.abs_diff(column as i32) <= 1 {
                '█'
            } else {
                ' '
            }
        })
        .collect()
}

struct Cycles<'a> {
    instructions: std::slice::Iter<'a, Instruction>,
    cycle: u32,
    x: i32,
    change: i32,
    delay: u32,
}

impl<'a> Cycles<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions: instructions.iter(),
            cycle: 0,
            x: 1,
            change: 0,
            delay: 0,
        }
    }
}

impl<'a> Iterator for Cycles<'a> {
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;
        if self.cycle > 240 {
            return None;
        }
        if self.delay == 0 {
            self.x += self.change;
            loop {
                match self.instructions.next() {
                    Some(&Noop) => self.delay += 1,
                    Some(&Addx(change)) => {
                        self.delay += 2;
                        self.change = change;
                        break;
                    }
                    None => break,
                }
            }
        }
        self.delay = self.delay.saturating_sub(1);
        Some((self.cycle, self.x))
    }
}

enum Instruction {
    Noop,
    Addx(i32),
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("noop") => Ok(Noop),
            Some("addx") => {
                let num = tokens.next().context("missing operand")?.parse()?;
                Ok(Addx(num))
            }
            _ => Err(Error::msg("invalid instruction")),
        }
    }
}
