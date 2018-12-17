#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::*;

const DATA: &'static str = include_str!("../../../data/13");

#[derive(Clone, Copy, Debug)]
enum Area { Track, Intersection, Slash, BackSlash }

#[derive(Clone, Copy, Debug)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn apply(&mut self, g: Area, turn: &mut Turn) {
        use self::Area::*;
        use self::Dir::*;

        *self = match (*self, g) {
            (_, Track) => return,
            (dir, Intersection) => turn.apply(dir),
            (Left, Slash) => Down,
            (Left, BackSlash) => Up,
            (Right, Slash) => Up,
            (Right, BackSlash) => Down,
            (Up, Slash) => Right,
            (Up, BackSlash) => Left,
            (Down, Slash) => Left,
            (Down, BackSlash) => Right,
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn { Left, Straight, Right }

impl Turn {
    fn apply(&mut self, cart: Dir) -> Dir {
        let out = match (*self, cart) {
            (Turn::Left, Dir::Up) => Dir::Left,
            (Turn::Left, Dir::Left) => Dir::Down,
            (Turn::Left, Dir::Down) => Dir::Right,
            (Turn::Left, Dir::Right) => Dir::Up,

            (Turn::Straight, cart) => cart,

            (Turn::Right, Dir::Up) => Dir::Right,
            (Turn::Right, Dir::Right) => Dir::Down,
            (Turn::Right, Dir::Down) => Dir::Left,
            (Turn::Right, Dir::Left) => Dir::Up,
        };

        *self = match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };

        out
    }
}

type Cart = ((isize, isize), Turn, Dir);

fn main() {
    let (grid, carts) = read_input();

    println!("Part 1: {:?}", process(&grid, carts.clone(), true));
    println!("Part 2: {:?}", process(&grid, carts.clone(), false));
}

fn read_input() -> (HashMap<(isize, isize), Area>, Vec<Cart>) {
    let mut grid = HashMap::new();
    let mut carts = vec![];

    for (y, line) in DATA.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let coords = (x as isize, y as isize);

            use self::Area::*;

            match ch {
                '+' => {
                    grid.insert(coords, Area::Intersection);
                }
                '/' => {
                    grid.insert(coords, Area::Slash);
                }
                '\\' => {
                    grid.insert(coords, Area::BackSlash);
                }
                '-' | '|' => {
                    grid.insert(coords, Area::Track);
                }
                '>' => {
                    grid.insert(coords, Area::Track);
                    carts.push((coords, Turn::Left, Dir::Right));
                }
                '^' => {
                    grid.insert(coords, Area::Track);
                    carts.push((coords, Turn::Left, Dir::Up));
                }
                '<' => {
                    grid.insert(coords, Area::Track);
                    carts.push((coords, Turn::Left, Dir::Left));
                }
                'v' => {
                    grid.insert(coords, Area::Track);
                    carts.push((coords, Turn::Left, Dir::Down));
                }
                _ => {}
            }
        }
    }

    (grid, carts)
}

fn process(grid: &HashMap<(isize, isize), Area>, mut carts: Vec<Cart>, first_crash: bool) -> (isize, isize) {
    loop {
        // Part 2 (find coordinates of last cart)
        if carts.len() == 1 {
            return carts[0].0;
        }

        let mut positions = HashSet::new();
        let mut removed = HashSet::new();

        carts.sort_by(|((ax, ay), _, _), ((bx, by), _, _)| (ay, ax).cmp(&(by, bx)));

        for (pos, _, _) in &mut carts {
            if !positions.insert(pos.clone()) {
                if first_crash {
                    return *pos;
                }

                removed.insert(*pos);
            }
        }

        // tick simulation
        for (_, (pos, turn, dir)) in carts.iter_mut().enumerate() {
            if removed.contains(pos) {
                continue;
            }

            positions.remove(pos);

            match *dir {
                Dir::Left => pos.0 -= 1,
                Dir::Right => pos.0 += 1,
                Dir::Up => pos.1 -= 1,
                Dir::Down => pos.1 += 1,
            }

            if !positions.insert(*pos) {
                if first_crash {
                    return *pos;
                }

                removed.insert(*pos);
                continue;
            }

            if let Some(g) = grid.get(pos).cloned() {
                dir.apply(g, turn);
            }
        }

        if !removed.is_empty() {
            carts = carts
                .into_iter()
                .filter(|c| !removed.contains(&c.0))
                .collect();
        }
    }
}
