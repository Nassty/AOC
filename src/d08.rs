use std::collections::{HashMap, VecDeque};

use aoc_toolkit::Day;
pub struct D8 {}

impl D8 {
    fn build_network(input: VecDeque<&str>) -> HashMap<String, (String, String)> {
        let mut h = HashMap::new();
        for line in input {
            let l1 = &line.replace("(", "");
            let l2 = &l1.replace(")", "");
            let s: Vec<_> = l2.split(" = ").collect();
            let source = s[0];
            let lr: Vec<_> = s[1].split(", ").collect();
            h.insert(source.into(), (lr[0].into(), lr[1].into()));
        }
        h
    }
}

impl Day for D8 {
    fn part1(&self, data: &str) -> String {
        let mut lines: VecDeque<_> = data.lines().collect();
        let instructions = lines.pop_front().unwrap();
        lines.pop_front().unwrap();
        let network = Self::build_network(lines);
        let mut current = "AAA".to_string();
        let mut path = 0;
        while current != *"ZZZ" {
            let instr = instructions.chars().collect::<Vec<_>>()[path % instructions.len()];
            let next = network.get(&current).unwrap();
            current = match instr {
                'L' => next.0.clone(),
                'R' => next.1.clone(),
                _ => unreachable!(),
            };
            path += 1;
        }
        path.to_string()
    }

    fn part2(&self, data: &str) -> String {
        let mut lines: VecDeque<_> = data.lines().collect();
        let instructions = lines.pop_front().unwrap();
        lines.pop_front().unwrap();
        let network = Self::build_network(lines);
        let starting_points: Vec<_> = network.keys().filter(|k| k.ends_with('A')).collect();
        let mut current_points: Vec<String> = Vec::new();
        for s in starting_points {
            current_points.push(s.clone());
        }

        let mut path = 0;

        while !current_points.iter().all(|x| x.ends_with('Z')) {
            let mut pivot = Vec::new();
            for current in current_points {
                let instr = instructions.chars().collect::<Vec<_>>()[path % instructions.len()];
                let next = network.get(&current.clone()).unwrap();
                pivot.push(match instr {
                    'L' => next.0.clone(),
                    'R' => next.1.clone(),
                    _ => unreachable!(),
                });
            }
            current_points = pivot;
            path += 1;
        }
        path.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D8 {};
        assert_eq!(
            "2",
            d.part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D8 {};
        assert_eq!(
            "6",
            d.part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
        );
    }
}
