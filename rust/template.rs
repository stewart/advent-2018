#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::*;

const DATA: &'static str = include_str!("../../../data/XX");

fn main() {
    let input = read_input();

    println!("{:?}", input);

    // println!("Part 1: {:?}", process(&grid, carts.clone(), true));
    // println!("Part 2: {:?}", process(&grid, carts.clone(), false));
}

fn read_input() -> usize {
    DATA.lines().count()
}
