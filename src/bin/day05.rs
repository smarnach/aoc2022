#![feature(let_chains)]

use anyhow::{Context, Error, Result};
use aoc2022::{parse_lines, read_input};
use lazy_static::lazy_static;
use regex::bytes::Regex;
use std::{
    collections::BTreeMap,
    str::{from_utf8, FromStr},
};

fn main() -> Result<()> {
    let (mut stacks, steps) = parse_input(&read_input!()?)?;
    let tops = stacks.clone().apply_all(&steps, CrateMover9000)?;
    println!("{}", from_utf8(&tops)?);
    let tops = stacks.apply_all(&steps, CrateMover9001)?;
    println!("{}", from_utf8(&tops)?);
    Ok(())
}

#[derive(Clone)]
struct Stacks {
    crates: BTreeMap<u8, Vec<u8>>,
}

#[derive(Clone, Copy)]
enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

use CraneModel::*;

impl Stacks {
    fn get_mut(&mut self, name: u8) -> Result<&mut Vec<u8>> {
        self.crates.get_mut(&name).context("invalid stack name")
    }

    fn apply(&mut self, step: &Step, model: CraneModel) -> Result<()> {
        for _ in 0..step.count {
            let c = self
                .get_mut(step.from)?
                .pop()
                .context("trying to move from empty stack")?;
            self.get_mut(step.to)?.push(c);
        }
        if let CrateMover9001 = model {
            let to = self.get_mut(step.to)?;
            let i = to.len() - step.count;
            to[i..].reverse();
        }
        Ok(())
    }

    fn apply_all(&mut self, steps: &[Step], model: CraneModel) -> Result<Vec<u8>> {
        for step in steps {
            self.apply(step, model)?;
        }
        Ok(self
            .crates
            .values()
            .flat_map(|s| s.last().cloned())
            .collect())
    }
}

impl FromStr for Stacks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let stacks: Vec<(usize, u8)> = lines
            .next()
            .context("empty stack spec")?
            .bytes()
            .enumerate()
            .filter(|&(_, name)| name != b' ')
            .collect();
        let mut crates: BTreeMap<_, _> = stacks.iter().map(|&(_, name)| (name, vec![])).collect();
        for line in lines {
            let line = line.as_bytes();
            for &(i, name) in &stacks {
                if let Some(&c) = line.get(i) && c != b' ' {
                    crates.get_mut(&name).unwrap().push(c);
                }
            }
        }
        Ok(Self { crates })
    }
}

struct Step {
    count: usize,
    from: u8,
    to: u8,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (.) to (.)").unwrap();
        }
        let cap = RE
            .captures(s.as_bytes())
            .context("invalid crane instruction")?;
        Ok(Step {
            count: from_utf8(&cap[1]).unwrap().parse()?,
            from: cap[2][0],
            to: cap[3][0],
        })
    }
}

fn parse_input(input: &str) -> Result<(Stacks, Vec<Step>)> {
    let (stacks, steps) = input
        .split_once("\n\n")
        .context("input must contain empty line separating stacks and steps")?;
    Ok((stacks.parse()?, parse_lines(steps).collect::<Result<_>>()?))
}
