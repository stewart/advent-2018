#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{HashMap,HashSet,BTreeSet};

const DATA: &'static str = include_str!("../../../data/07");

type Input = HashMap<char, Vec<char>>;

fn main() {
    let input = read_input();

    println!("Part 01: {}", part1(&input));
    println!("Part 02: {}", part2(&input, 60, 5));
}

fn read_input() -> Input {
    DATA
        .lines()
        .map(|line| line.split(" ").collect())
        .map(|line: Vec<&str>| (line[1].chars().next().unwrap(), line[7].chars().next().unwrap()))
        .fold(HashMap::new(), |mut acc, (x, y)| {
            acc.entry(x).or_default();
            acc.entry(y).or_default().push(x);
            acc
        })
}

// In what order should the steps in your instructions be completed?
fn part1(nodes: &Input) -> String {
    let mut left: BTreeSet<char> = nodes.keys().cloned().collect();
    let mut satisfied = HashSet::new();
    let mut output = String::new();

    while !left.is_empty() {
        for node in left.iter().cloned() {
            let available = match nodes.get(&node) {
                Some(node) => node.iter().all(|dep| satisfied.contains(dep)),
                None => true,
            };

            if available {
                output.push(node);
                satisfied.insert(node);
                left.remove(&node);

                break;
            }
        }
    }

    output
}

// With 5 workers and the 60+ second step durations described above,
// how long will it take to complete all of the steps?
fn part2(nodes: &Input, base: usize, worker_count: usize) -> usize {
    let mut left: BTreeSet<char> = nodes.keys().cloned().collect();
    let mut satisfied = HashSet::new();

    let mut workers: Vec<Worker> = std::iter::repeat(())
        .map(|_| Worker::new())
        .take(worker_count)
        .collect();

    let mut seconds = 0;

    loop {
        seconds += 1;

        let mut idle_workers = vec![];

        for worker in &mut workers {
            worker.tick();

            if worker.work == 0 {
                if let Some(work) = worker.current.take() {
                    satisfied.insert(work);
                }

                idle_workers.push(worker);
            }
        }

        // if nobody has any work to do, we're done!
        if idle_workers.len() == worker_count && left.is_empty() {
            break;
        }

        // if everyone's busy, check back on the next tick.
        if idle_workers.is_empty() {
            continue;
        }

        let available_work: Vec<char> = left.iter().cloned().collect();

        for node in available_work {
            // if we've assign all workers, don't bother scanning available work
            if idle_workers.is_empty() {
                break;
            }

            // find available satisfied nodes
            let available = match nodes.get(&node) {
                Some(node) => node.iter().all(|node| satisfied.contains(node)),
                None => true,
            };

            if available {
                // assign available work to idle worker
                if let Some(worker) = idle_workers.pop() {
                    worker.work = base + (node as usize) - (b'A' as usize) + 1;
                    worker.current = Some(node);
                    left.remove(&node);
                }
            }
        }
    }

    // our loop necessarily starts at tick 1, so fix that at return time
    seconds - 1
}

#[derive(Debug)]
struct Worker {
  // what is currently being worked on
  current: Option<char>,

  // how much work needs to be done
  work: usize,
}

impl Worker {
  fn new() -> Worker {
    Worker { current: None, work: 0 }
  }

  fn tick(&mut self) {
    self.work = self.work.saturating_sub(1);
  }
}
