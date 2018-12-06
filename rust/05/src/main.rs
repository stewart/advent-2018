#![allow(dead_code, unused_imports, unused_variables)]

const DATA: &'static str = include_str!("../../../data/05");

fn main() {
    let chars = DATA.trim();

    println!("Part 01: {}", part1(&chars));
    println!("Part 02: {}", part2(&chars));
}

fn part1(input: &str) -> usize {
    react(input).len()
}

fn part2(input: &str) -> usize {
    let alphabet = (b'a'..=b'z').map(|n| n as char);

    alphabet
        .map(|ch| {
            let stripped: String = input
                .chars()
                .filter(|i| !i.eq_ignore_ascii_case(&ch))
                .collect();

            react(&stripped).len()
        }).min().unwrap()
}

fn react(polymer: &str) -> Vec<char> {
    polymer
        .chars()
        .fold(vec![], |mut acc, ch| {
            match acc.pop() {
                Some(last) => if !matches(ch, last) {
                    acc.push(last);
                    acc.push(ch);
                },
                None => acc.push(ch)
            }
            acc
        })
}

fn matches(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}
