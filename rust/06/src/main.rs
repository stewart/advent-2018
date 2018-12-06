#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{HashMap,HashSet};

const DATA: &'static str = include_str!("../../../data/06");

fn main() {
    let points: Vec<(isize, isize)> = DATA
        .lines()
        .map(|line| line.split(", ").map(|n| n.parse::<isize>().unwrap()))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .collect();

    println!("Part 01: {}", part1(&points));
    // println!("Part 02: {}", part2(DATA));
}

fn part1(points: &Vec<(isize, isize)>) -> usize {
    let mut areas = points.iter().fold(HashMap::new(), |mut acc, xy| {
        acc.insert(*xy, 0);
        acc
    });

    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    let mut infinites = HashSet::<(isize, isize)>::new();

    for x in (min_x - 2)..=(max_x + 2) {
        for y in (min_x - 2)..=(max_y + 2) {
            let mut min_distance = 10_000;
            let mut closest = None;

            for &(a, b) in areas.keys() {
                let distance = (a - x).abs() + (b - y).abs();

                if distance <= min_distance {
                    min_distance = distance;
                    closest = Some((a, b))
                } else if distance == min_distance {
                    closest = None;
                }
            }

            if let Some(coords) = closest {
                *areas.entry(coords).or_insert(0) += 1;

                if x == min_x - 2 || x == max_x + 2 || y == min_y - 2 || y == max_x + 2 {
                    infinites.insert(coords);
                }
            }
        }
    }

    let max = areas
        .iter()
        .filter(|area| !infinites.contains(&area.0))
        .max_by_key(|&(a, b)| b);

    *max.unwrap().1
}

fn part2(input: &str) -> usize {
    0
}
