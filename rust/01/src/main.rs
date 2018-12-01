use std::collections::HashMap;

const INPUT: &'static str = include_str!("../data");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut current_frequency: isize = 0;

    for (op, num) in INPUT.lines().map(|line| line.split_at(1)) {
        let modulation: isize = num.parse().expect("Unable to parse number");

        let new_frequency = match op {
            "+" => { current_frequency + modulation },
            "-" => { current_frequency - modulation },
            _ => { unreachable!() }
        };

        current_frequency = new_frequency;
    }

    println!("Part 1: {}", current_frequency);
}

fn part2() {
    let mut current_frequency: isize = 0;
    let mut frequencies_seen = HashMap::new();

    frequencies_seen.insert(0, 1);

    for (op, num) in INPUT.lines().map(|line| line.split_at(1)).cycle() {
        let modulation: isize = num.parse().expect("Unable to parse number");

        let new_frequency = match op {
            "+" => { current_frequency + modulation },
            "-" => { current_frequency - modulation },
            _ => { unreachable!() }
        };

        let entry = frequencies_seen.entry(new_frequency).or_insert(0);
        *entry += 1;

        if *entry == 2 {
            println!("Part 2: {}", new_frequency);
            break;
        }

        current_frequency = new_frequency;
    }
}
