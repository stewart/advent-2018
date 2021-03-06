#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{HashMap, HashSet};

const DATA: &'static str = include_str!("../../../data/02");

type Input = &'static str;

fn main() {
    println!("Part 01: {}", part1(DATA));
    println!("Part 02: {}", part2(DATA).unwrap());
}

fn part1(input: Input) -> usize {
    let values = input
        .lines()
        .map(|line| line.chars().collect())
        .map(|chars: Vec<char>| {
            let mut map = HashMap::new();

            for ch in chars {
                let entry = map.entry(ch).or_insert(0);
                *entry += 1;
            }

            map
        });

    let number_of_values_with_two = values
        .clone()
        .filter(|map| map.values().any(|&value| value == 2))
        .count();

    let number_of_values_with_three = values
        .clone()
        .filter(|map| map.values().any(|&value| value == 3))
        .count();

    number_of_values_with_two * number_of_values_with_three
}

fn part2(input: Input) -> Option<String> {
    let codes: Vec<&str> = input.lines().collect();

    for a in &codes {
        for b in &codes {
            let count = a.chars().zip(b.chars()).filter(|(x, y)| x != y).count();

            if count == 1 {
                return Some(
                    a.chars()
                        .zip(b.chars())
                        .filter(|(x, y)| x == y)
                        .map(|(x, y)| x)
                        .collect(),
                );
            }
        }
    }

    None
}
