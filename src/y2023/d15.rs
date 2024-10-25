use aoc_toolkit::Day;
use std::{ops::Mul, str::FromStr};

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |current, char| (current + *char as usize).mul(17) % 256)
}

fn parse(input: &str) -> usize {
    input.split_terminator(',').map(hash).sum::<usize>()
}

#[derive(Debug, PartialEq)]
enum Operation {
    Remove,
    Swap(usize),
}

#[derive(Debug, PartialEq)]
struct Instruction {
    source: String,
    label: usize,
    operation: Operation,
}

impl Instruction {
    fn split(input: &str) -> (&str, &str, &str) {
        let mut p = 0;
        for i in input.chars() {
            if !i.is_alphabetic() {
                break;
            }
            p += 1;
        }
        (&input[0..p], &input[p..=p], &input[p + 1..])
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source, op, focal) = Self::split(s);
        let label = hash(source);
        let operation = match op {
            "-" => Operation::Remove,
            "=" => Operation::Swap(focal.parse().expect("number")),
            k => return Err(format!("can't parse {} operation in {s}", k)),
        };
        Ok(Self {
            label,
            operation,
            source: source.to_string(),
        })
    }
}

fn parse2(input: &str) -> usize {
    let mut boxes: [Vec<(String, usize)>; 256] = [const { Vec::new() }; 256];
    let instructions = input
        .split_terminator(",")
        .map(|x| x.parse::<Instruction>().expect("parse"))
        .collect::<Vec<_>>();
    for Instruction {
        label,
        source,
        operation,
    } in instructions
    {
        let b = &mut boxes[label];
        match operation {
            Operation::Remove => b.retain(|(x, _)| *x != source),
            Operation::Swap(focal) => {
                if b.iter()
                    .filter(|(x, _)| *x == source)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    b.push((source.to_string(), focal));
                } else {
                    boxes[label] = b
                        .iter()
                        .map(|(x, y)| (x.clone(), if *x == source { focal } else { *y }))
                        .collect::<Vec<_>>();
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_index, _box)| {
            _box.iter()
                .enumerate()
                .map(move |(index, (_, focal))| (box_index + 1) * (index + 1) * focal)
        })
        .sum()
}

pub struct D15 {}
impl Day for D15 {
    fn part1(&self, input: &str) -> String {
        parse(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse2(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(
            parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320,
        );
    }
    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
        assert_eq!(97, hash("qp=3"));
        assert_eq!(14, hash("qp-"));
        assert_eq!(180, hash("pc=4"));
        assert_eq!(9, hash("ot=9"));
        assert_eq!(197, hash("ab=5"));
        assert_eq!(48, hash("pc-"));
        assert_eq!(214, hash("pc=6"));
        assert_eq!(231, hash("ot=7"));
        assert_eq!(0, hash("rn"));
        assert_eq!(0, hash("cm"));
        assert_eq!(1, hash("qp"));
        assert_eq!(3, hash("pc"));
        assert_eq!(3, hash("ot"));
        assert_eq!(3, hash("ab"));
    }
    #[test]
    fn test_parse_instr() {
        assert_eq!(
            Instruction {
                label: 0,
                source: "rn".to_string(),
                operation: Operation::Swap(1),
            },
            "rn=1".parse::<Instruction>().expect("parse"),
        );
    }
    #[test]
    fn test_parse2() {
        assert_eq!(
            parse2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145,
        );
    }
}
