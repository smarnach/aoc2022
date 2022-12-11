use anyhow::{Context, Error, Result};
use aoc2022::{parse_split_comma, read_input};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Reverse, str::FromStr};

fn main() -> Result<()> {
    let input = read_input!()?;
    let mut troop: Troop = input.parse()?;
    let mut troop2 = troop.clone();
    troop.play_rounds(20, 3);
    println!("{}", troop.monkey_business());
    troop2.play_rounds(10_000, 1);
    println!("{}", troop2.monkey_business());
    Ok(())
}

#[derive(Clone, Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
    modulo: u64,
}

impl Troop {
    fn play_rounds(&mut self, count: usize, divide_by: u64) {
        for _ in 0..count {
            for i in 0..self.monkeys.len() {
                for (j, item) in self.monkeys[i].play(divide_by, self.modulo) {
                    self.monkeys[j].items.push(item);
                }
            }
        }
    }

    fn monkey_business(&mut self) -> u64 {
        self.monkeys.sort_by_key(|monkey| Reverse(monkey.activity));
        self.monkeys[0].activity * self.monkeys[1].activity
    }
}

impl FromStr for Troop {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let monkeys = input
            .split("\n\n")
            .map(|block| block.parse())
            .collect::<Result<Vec<Monkey>>>()?;
        let modulo = monkeys.iter().map(|monkey| monkey.divisor).product::<u64>();
        Ok(Self { monkeys, modulo })
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    next_monkeys: [usize; 2],
    activity: u64,
}

impl Monkey {
    fn play(&mut self, divide_by: u64, modulo: u64) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|item| {
                self.activity += 1;
                let item = self.operation.apply(item) / divide_by % modulo;
                let condition = item % self.divisor == 0;
                (self.next_monkeys[condition as usize], item)
            })
            .collect()
    }
}

const MONKEY_REGEX: &str = r#"^Monkey (?P<index>\d+):
  Starting items: (?P<items>.*)
  Operation: new = old (?P<operator>\+|\*) (?P<operand>old|\d+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<true_monkey>\d+)
    If false: throw to monkey (?P<false_monkey>\d+)\n?$"#;

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(MONKEY_REGEX).unwrap();
        }
        let cap = RE.captures(s).context("invalid monkey specification")?;
        let items = parse_split_comma(&cap["items"]).collect::<Result<Vec<u64>>>()?;
        let operation = Operation::new(&cap["operator"], &cap["operand"])?;
        Ok(Self {
            items,
            operation,
            divisor: cap["divisor"].parse()?,
            next_monkeys: [cap["false_monkey"].parse()?, cap["true_monkey"].parse()?],
            activity: 0,
        })
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn new(operator: &str, operand: &str) -> Result<Self> {
        match operand {
            "old" => match operator {
                "+" => Ok(Operation::Mul(2)),
                "*" => Ok(Operation::Square),
                _ => Err(Error::msg("invalid operator")),
            },
            _ => {
                let operand: u64 = operand.parse()?;
                match operator {
                    "+" => Ok(Operation::Add(operand)),
                    "*" => Ok(Operation::Mul(operand)),
                    _ => Err(Error::msg("invalid operator")),
                }
            }
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match *self {
            Operation::Add(operand) => old + operand,
            Operation::Mul(operand) => old * operand,
            Operation::Square => old * old,
        }
    }
}
