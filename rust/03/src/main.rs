#![allow(dead_code, unused_imports, unused_variables)]

extern crate regex;

use std::collections::{HashMap,HashSet};
use regex::Regex;

const DATA: &'static str = include_str!("../../../data/03");

#[derive(Copy, Clone, Debug)]
struct Claim {
    id: usize,
    left: usize,
    right: usize,
    width: usize,
    height: usize
}

type Input = Vec<Claim>;

fn main() {
    let regex = Regex::new(r"^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let claims: Vec<_> = DATA
        .lines()
        .map(|line| regex.captures(line).unwrap())
        .map(|c| {
            Claim {
                id: c[1].parse().unwrap(),
                left: c[2].parse().unwrap(),
                right: c[3].parse().unwrap(),
                width: c[4].parse().unwrap(),
                height: c[5].parse().unwrap(),
            }
        })
        .collect();

    println!("Part 01: {}", part1(claims.clone()));
    println!("Part 02: {}", part2(claims.clone()));
}

fn part1(claims: Vec<Claim>) -> usize {
    let mut squares = HashSet::new();
    let mut repeats = HashSet::new();

    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.right..(claim.right + claim.height) {
                if !squares.insert((x, y)) {
                    repeats.insert((x, y));
                }
            }
        }
    }

    repeats.len()
}

fn part2(claims: Input) -> usize {
    let mut squares = HashMap::new();
    let mut not_conflicting: HashSet<usize> = claims.iter().map(|claim| claim.id).collect();

    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.right..(claim.right + claim.height) {
                let entry = squares.entry((x, y)).or_insert(claim.id);

                if *entry != claim.id {
                    not_conflicting.remove(&claim.id);
                    not_conflicting.remove(entry);
                }
            }
        }
    }

    *not_conflicting.iter().next().unwrap()
}
