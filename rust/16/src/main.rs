#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::*;

const DATA: &'static str = include_str!("../../../data/16");

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Device {
    registers: [usize; 4],
}

impl Device {
    fn parse(input: &str) -> Option<Device> {
        let parsed: Vec<usize> = input
            .trim_matches(|ch| ch == '[' || ch == ']')
            .split(", ")
            .map(|s: &str| str::parse(s).unwrap())
            .collect();

        match parsed.as_slice() {
            [a, b, c, d] => Some(Device { registers: [*a, *b, *c, *d] }),
            _ => None
        }
    }

    fn register(&mut self, num: usize) -> Option<&mut usize> {
        self.registers.get_mut(num)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OpCode {
    Addr, Addi,
    Mulr, Muli,
    Banr, Bani,
    Borr, Bori,
    Setr, Seti,
    Gtir, Gtri,
    Gtrr, Eqir,
    Eqri, Eqrr,
}

impl OpCode {
    /// Iterate over all variants.
    fn variants() -> impl Iterator<Item = OpCode> {
        use self::OpCode::*;

        [
            Addr, Addi,
            Mulr, Muli,
            Banr, Bani,
            Borr, Bori,
            Setr, Seti,
            Gtir, Gtri,
            Gtrr, Eqir,
            Eqri, Eqrr,
        ].into_iter().cloned()
    }

    fn apply(&self, d: &mut Device, inputs: &[usize; 2], o: usize) -> Option<()> {
        use self::OpCode::*;

        let [a, b] = *inputs;

        *d.register(o)? = match *self {
            Addr => *d.register(a)? + *d.register(b)?,
            Addi => *d.register(a)? + b,

            Mulr => *d.register(a)? * *d.register(b)?,
            Muli => *d.register(a)? * b,

            Banr => *d.register(a)? & *d.register(b)?,
            Bani => *d.register(a)? & b,

            Borr => *d.register(a)? | *d.register(b)?,
            Bori => *d.register(a)? | b,

            Setr => *d.register(a)?,
            Seti => a,

            Gtir => if a > *d.register(b)? { 1 } else { 0 },
            Gtri => if *d.register(a)? > b { 1 } else { 0 },
            Gtrr => if *d.register(a)? > *d.register(b)? { 1 } else { 0 },

            Eqir => if a == *d.register(b)? { 1 } else { 0 },
            Eqri => if *d.register(a)? == b { 1 } else { 0 },
            Eqrr => if *d.register(a)? == *d.register(b)? { 1 } else { 0 },
        };

        Some(())
    }
}

#[derive(Debug)]
pub struct Instruction {
    op_code: usize,
    inputs: [usize; 2],
    output: usize,
}

impl Instruction {
    pub fn parse(input: &str) -> Option<Instruction> {
        let mut input = input.split(" ").flat_map(|d| str::parse(d));

        let op_code = input.next()?;
        let inputs = [input.next()?, input.next()?];
        let output = input.next()?;

        Some(Instruction { op_code, inputs, output })
    }
}

#[derive(Debug)]
struct Test {
    before: Device,
    instruction: Instruction,
    after: Device,
}

impl Test {
    fn parse(input: &[&str]) -> Option<Test> {
        let before = input[0];
        let after = input[2];

        let before = Device::parse(before.split(": ").nth(1)?.trim())?;
        let instruction = Instruction::parse(input[1])?;
        let after = Device::parse(after.split(": ").nth(1)?.trim())?;

        Some(Test { before, instruction, after })
    }
}

fn main() {
    let lines: Vec<_> = DATA.lines().collect();
    let (first, second) = lines.split_at(3095);

    let tests: Vec<Test> = first.chunks(4).flat_map(Test::parse).collect();
    let instructions: Vec<Instruction> = second.into_iter().cloned().flat_map(Instruction::parse).collect();

    println!("Part 1: {:?}", part1(&tests));
    println!("Part 2: {:?}", part2(&tests, &instructions));
}

fn part1(tests: &Vec<Test>) -> usize {
    let mut score = 0;

    for test in tests {
        let mut matches = HashSet::new();

        for op in OpCode::variants() {
            let mut device = test.before.clone();
            op.apply(&mut device, &test.instruction.inputs, test.instruction.output);

            if device == test.after {
                matches.insert(op);
            }
        }

        if matches.len() >= 3 {
            score += 1;
        }
    }

    score
}

fn part2(tests: &Vec<Test>, instructions: &Vec<Instruction>) -> usize {
    let mut mapping: HashMap<usize, HashSet<OpCode>> = HashMap::new();

    // initially, all codes are possible for all numbers
    for n in 0..16 {
        for op in OpCode::variants() {
            mapping.entry(n).or_default().insert(op);
        }
    }

    // use tests to reduce possibility space 
    for test in tests {
        let mut matches = HashSet::new();

        for op in OpCode::variants() {
            let mut device = test.before.clone();
            op.apply(&mut device, &test.instruction.inputs, test.instruction.output);

            if device == test.after {
                matches.insert(op);
            } else {
                // does not match, can be removed from mapping
                if let Some(codes) = mapping.get_mut(&test.instruction.op_code) {
                    codes.remove(&op);
                }
            }
        }

        // if only one match, this is the only possible opcode for this digit
        if matches.len() == 1 {
            if let Some(op) = matches.into_iter().next() {
                if let Some(codes) = mapping.get_mut(&test.instruction.op_code) {
                    codes.clear();
                    codes.insert(op);
                }
            }
        }
    }

    // regress mappings recursively to resolve
    {
        let mut identified = vec![];
        let mut current = 0;

        extract_identified(&mapping, &mut identified);

        while current != identified.len() {
            current = identified.len();

            for (identified_num, identified_code) in &identified {
                for (num, codes) in mapping.iter_mut() {
                    if num == identified_num {
                        codes.clear();
                        codes.insert(*identified_code);
                        continue;
                    }

                    codes.remove(&identified_code);
                }
            }

            identified.clear();
            extract_identified(&mapping, &mut identified);
        }
    }

    let mapping: HashMap<usize, OpCode> = mapping
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().next().unwrap()))
        .collect();

    let mut device = Device::default();

    for instruction in instructions {
        let op = mapping.get(&instruction.op_code).cloned().unwrap();
        op.apply(&mut device, &instruction.inputs, instruction.output);
    }

    *device.register(0).unwrap()
}

fn extract_identified(input: &HashMap<usize, HashSet<OpCode>>, output: &mut Vec<(usize, OpCode)>) {
    for (key, value) in input.iter().filter(|(key, value)| value.len() == 1) {
        output.push((*key, value.iter().cloned().next().unwrap()));
    }
}
