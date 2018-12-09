#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{BTreeSet,HashMap,HashSet};

const DATA: &'static str = include_str!("../../../data/08");

type Input = Vec<usize>;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(input: &mut impl Iterator<Item=usize>) -> Option<Node> {
        let children = match input.next() {
            Some(num) => num,
            None => { return None }
        };

        let metadata = input.next().expect("No metadata");

        let mut node = Node {
            children: vec![],
            metadata: vec![],
        };

        for _ in 0..children {
            node.children.extend(Node::parse(input));
        }

        node.metadata.extend(input.take(metadata as usize));

        Some(node)
    }

    fn metadata_sum_check(&self) -> usize {
        let metadata = self.metadata.iter().cloned();
        let children = self.children.iter();

        metadata.sum::<usize>() + children.map(Node::metadata_sum_check).sum::<usize>()
    }

    fn weird_indexing_sum_check(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().cloned().sum();
        } else {
            return self.metadata
                .iter()
                .cloned()
                .filter(|n| *n > 0)
                .map(|idx| {
                    match self.children.get(idx - 1) {
                        Some(child) => child.weird_indexing_sum_check(),
                        None => 0
                    }
                }).sum()
        }
    }
}

fn main() {
    let input = read_input();
    let mut iter = input.iter().cloned();

    let tree = Node::parse(&mut iter).expect("Unable to parse");

    println!("Part 01: {}", part1(&tree));
    println!("Part 02: {}", part2(&tree));
}

fn read_input() -> Input {
    DATA
        .trim()
        .split(" ")
        .map(|n| n.parse().expect("Unable to parse number"))
        .collect()
}

fn part1(root: &Node) -> usize {
    Node::metadata_sum_check(root)
}

fn part2(root: &Node) -> usize {
    Node::weird_indexing_sum_check(root)
}
