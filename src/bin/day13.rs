#![feature(array_chunks, let_chains)]

use anyhow::{Context, Error, Result};
use aoc2022::read_input;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, str::FromStr};

fn main() -> Result<()> {
    let input = read_input!()?;
    let mut packets = parse_input(&input)?;
    let index_sum: usize = (1usize..)
        .zip(packets.array_chunks())
        .filter_map(|(i, [a, b])| (a < b).then_some(i))
        .sum();
    println!("{index_sum}");
    packets.sort_unstable();
    let index_product =
        (binary_search(&packets, "[[2]]")? + 1) * (binary_search(&packets, "[[6]]")? + 2);
    println!("{index_product}");
    Ok(())
}

fn binary_search(packets: &[Packet], p: &str) -> Result<usize> {
    packets
        .binary_search(&p.parse()?)
        .err()
        .context("input contains divider packet")
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Int(a), Int(b)) => a.cmp(b),
            (List(v), List(w)) => v.cmp(w),
            (&Int(a), List(w)) => vec![Packet::Int(a)].cmp(w),
            (List(v), &Int(b)) => v.cmp(&vec![Packet::Int(b)]),
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use ParserState::*;
        let mut stack = vec![vec![]];
        let mut state = StartItem;
        for token in Tokens::new(line) {
            match (state, token) {
                (StartItem, "[") => stack.push(vec![]),
                (_, "]") => {
                    let list = Packet::List(stack.pop().unwrap());
                    stack.last_mut().context("unexpected ]")?.push(list);
                    state = Finished;
                }
                (StartItem, _) => {
                    let packet = Packet::Int(token.parse()?);
                    stack.last_mut().unwrap().push(packet);
                    state = Finished;
                }
                (Finished, ",") => state = StartItem,
                _ => return Err(Error::msg("unexpected token")),
            }
        }
        if stack.len() == 1 && stack.first().unwrap().len() == 1 {
            let mut v = stack.pop().unwrap();
            let item = v.pop().unwrap();
            Ok(item)
        } else {
            Err(Error::msg("parse error"))
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ParserState {
    StartItem,
    Finished,
}

struct Tokens<'a> {
    line: &'a str,
}

impl<'a> Tokens<'a> {
    fn new(line: &'a str) -> Self {
        Self { line }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d+|,|\[|\]").unwrap();
        }
        let m = RE.find(self.line)?;
        self.line = &self.line[m.end()..];
        Some(m.as_str())
    }
}

fn parse_input(input: &str) -> Result<Vec<Packet>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::parse)
        .collect()
}
