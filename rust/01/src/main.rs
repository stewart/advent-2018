use std::collections::HashSet;

const INPUT: &'static str = include_str!("../data");

fn main() {
    part1();
    part2();
}

fn part1() {
    let result: isize = INPUT
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum();

    println!("Part 1: {}", result);
}

fn part2() {
    let mut current_frequency: isize = 0;
    let mut frequencies_seen = HashSet::new();

    frequencies_seen.insert(0);

    for modulation in INPUT.lines().cycle() {
        let modulation: isize = modulation.parse().expect("NaN");
        current_frequency += modulation;

        let was_not_set = frequencies_seen.insert(current_frequency);

        if was_not_set == false {
            break;
        }
    }

    println!("Part 2: {}", current_frequency);
}
