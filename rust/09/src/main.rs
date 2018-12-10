#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::VecDeque;

const PLAYERS: usize = 455;
const LAST_SCORE: usize = 7122300;

fn main() {
    println!("Part 01: {}", score(455, 71223));
    println!("Part 02: {}", score(455, 7122300));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn score(players: usize, final_score: usize) -> usize {
    let mut scores = vec![0; players];

    let mut marbles = VecDeque::new();
    marbles.push_back(0);

    for marble in 1..=final_score {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let back = marbles.pop_back().unwrap();
                marbles.push_front(back);
            }

            scores[marble % players] += marble + marbles.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let front = marbles.pop_front().unwrap();
                marbles.push_back(front);
            }

            marbles.push_front(marble);
        }
    }

    scores.into_iter().max().unwrap()
}
