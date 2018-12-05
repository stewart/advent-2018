#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{HashMap,HashSet};

const DATA: &'static str = include_str!("../../../data/05");

type Input = Vec<char>;

fn main() {
    let chars: Vec<char> = DATA.trim().chars().collect();

    println!("Part 01: {}", part1(&chars));
    println!("Part 02: {}", part2(&chars));
}

fn part1(input: &Input) -> usize {
    0
}

fn part2(input: &Input) -> usize {
    0
}
