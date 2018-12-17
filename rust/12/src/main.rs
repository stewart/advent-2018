#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::*;

const DATA: &'static str = include_str!("../../../data/12");

fn main() {
    let input: Vec<&str> = DATA.trim().lines().collect();
    let (first, rest) = input.split_at(2);

    let initial_state: String = first[0][15..].to_string();

    let rules: HashMap<&str, bool> = rest
        .iter()
        .map(|line| (&line[0..5], line.ends_with('#')))
        .collect();

    println!("Part 01: {}", process(&initial_state, &rules, 20));
    println!("Part 02: {}", process(&initial_state, &rules, 50_000_000_000));
}

fn process(initial: &str, rules: &HashMap<&str, bool>, generations: usize) -> isize {
    let mut state = format!("...{}...", initial);
    let mut last = 0;
    let mut known_scores = HashMap::new();

    for gen in 1..=generations {
        let mut new_state = String::from("...");

        for idx in 2..state.len() - 2 {
            let slice = &state[idx - 2..=idx + 2];
            new_state.push(
                match rules.get(slice) {
                    Some(true) => '#',
                    _ => '.',
                }
            )
        }
        new_state.push_str("...");
        state = new_state;

        let score = state
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum();

        let entry = known_scores.entry(score - last).or_insert(0);

        if *entry > 2 {
            return (generations - gen) as isize * (score - last) + score;
        } else {
            *entry += 1;
        }

        last = score
    }

    last
}
