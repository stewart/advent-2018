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
    println!("Part 02: {}", part2(&points));
}

fn part1(points: &Vec<(isize, isize)>) -> usize {
    let mut areas = points.iter().fold(HashMap::new(), |mut acc, xy| {
        acc.insert(*xy, 0);
        acc
    });

    let ((min_x, max_x), (min_y, max_y)) = bounds(&points, 2);

    let mut infinites = HashSet::<(isize, isize)>::new();

    for x in min_x..=max_x {
        for y in min_x..=max_y {
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

                if x == min_x || x == max_x || y == min_y || y == max_x {
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

fn part2(input: &Vec<(isize, isize)>) -> usize {
    let ((min_x, max_x), (min_y, max_y)) = bounds(input, 2);

    let limit = 10_000;
    let mut count = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let total_distance: isize = input
                .iter()
                .map(|(a, b)| (a - x).abs() + (b - y).abs())
                .sum();

            if total_distance < limit {
                count += 1;
            }
        }
    }

    count
}

fn bounds(points: &Vec<(isize, isize)>, margin: isize) -> ((isize, isize), (isize, isize)) {
    let min_x = *points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *points.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    ((min_x - margin, max_x + margin), (min_y - margin, max_y + margin))
}
