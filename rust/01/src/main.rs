use std::collections::HashSet;

const INPUT: &'static str = include_str!("../../../data/01");

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> isize {
    INPUT
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum()
}

fn part2() -> isize {
    let mut frequency: isize = 0;
    let mut frequencies_seen = HashSet::new();

    frequencies_seen.insert(0);

    for modulation in INPUT.lines().cycle() {
        let modulation: isize = modulation.parse().expect("NaN");
        frequency += modulation;

        let was_not_set = frequencies_seen.insert(frequency);

        if was_not_set == false {
            break;
        }
    }

    frequency
}
