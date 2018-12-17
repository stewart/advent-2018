#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::*;

const COUNT: usize = 513401;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let mut recipes = vec![3, 7];
    let mut first = 0;
    let mut second = 1;

    while recipes.len() < COUNT + 10 {
        tick(&mut recipes, &mut first, &mut second);
    }

    recipes[COUNT..COUNT+10]
        .iter()
        .map(|n| n.to_string())
        .collect()
}

fn part2() -> usize {
    let mut recipes = vec![3, 7];
    let mut first = 0;
    let mut second = 1;

    let sequence = [5, 1, 3, 4, 0, 1];
    let s_len = sequence.len();

    let answer;

    loop {
        let score = tick(&mut recipes, &mut first, &mut second);
        let r_len = recipes.len();

        if r_len > s_len {
            if &recipes[r_len-s_len..r_len] == sequence {
                answer = r_len - s_len;
                break;
            }
        }

        if score >= 10 && r_len > s_len + 1 {
            if &recipes[r_len - 1 - s_len..r_len - 1] == sequence {
                answer = r_len - 1 - s_len;
                break;
            }
        }
    }

    answer
}

fn tick(recipes: &mut Vec<usize>, first: &mut usize, second: &mut usize) -> usize {
    let score = recipes[*first] + recipes[*second];

    if score >= 10 {
        recipes.push(score / 10);
    }

    recipes.push(score % 10);

    *first = (*first + 1 + recipes[*first]) % recipes.len();
    *second = (*second + 1 + recipes[*second]) % recipes.len();

    score
}
