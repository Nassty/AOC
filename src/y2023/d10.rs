#![allow(suspicious_double_ref_op)]
use std::collections::HashMap;

use aoc_toolkit::Day;
pub struct D10 {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pipe {
    pos: (i32, i32),
    connections: Vec<(i32, i32)>,
    starting: bool,
}

impl Pipe {
    fn starting(pos: (i32, i32)) -> Self {
        Self {
            connections: Vec::default(),
            pos,
            starting: true,
        }
    }
    fn next(&self, my_map: &HashMap<(i32, i32), Self>) -> Vec<Self> {
        self.connections
            .iter()
            .map(|k| my_map.get(k))
            .filter(|k| k.is_some())
            .map(|k| k.unwrap().clone())
            .collect()
    }
    fn from_char_with_ground(s: char, pos: (i32, i32)) -> Option<Self> {
        let (a, b) = pos;
        let connections = match s {
            '|' => vec![(a - 1, b), (a + 1, b)],
            '-' => vec![(a, b - 1), (a, b + 1)],
            'L' => vec![(a - 1, b), (a, b + 1)],
            'J' => vec![(a - 1, b), (a, b - 1)],
            'F' => vec![(a, b + 1), (a + 1, b)],
            '7' => vec![(a, b - 1), (a + 1, b)],
            '.' => vec![],
            'S' => return Some(Pipe::starting(pos)),
            k => unreachable!("{:?}", k),
        };
        Some(Self {
            pos,
            connections,
            starting: false,
        })
    }
    fn from_char(s: char, pos: (i32, i32)) -> Option<Self> {
        let (a, b) = pos;
        let connections = match s {
            '|' => vec![(a - 1, b), (a + 1, b)],
            '-' => vec![(a, b - 1), (a, b + 1)],
            'L' => vec![(a - 1, b), (a, b + 1)],
            'J' => vec![(a - 1, b), (a, b - 1)],
            'F' => vec![(a, b + 1), (a + 1, b)],
            '7' => vec![(a, b - 1), (a + 1, b)],
            '.' => return None,
            'S' => return Some(Pipe::starting(pos)),
            k => unreachable!("{:?}", k),
        };
        Some(Self {
            pos,
            connections,
            starting: false,
        })
    }
}

fn parse_a(input: &str) -> HashMap<(i32, i32), Pipe> {
    let mut output = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let coord = (row as i32, col as i32);
            if let Some(k) = Pipe::from_char(c, coord) {
                output.insert(coord, k);
            }
        }
    }
    output
}

fn parse_b(input: &str) -> HashMap<(i32, i32), Pipe> {
    let mut output = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let coord = (row as i32, col as i32);
            if let Some(k) = Pipe::from_char_with_ground(c, coord) {
                output.insert(coord, k);
            }
        }
    }
    output
}

impl Day for D10 {
    fn part1(&self, data: &str) -> String {
        let my_map = parse_a(data);

        let start: Vec<_> = my_map.values().filter(|x| x.starting).collect();
        let mut start = start.first().unwrap().clone().clone();
        let mut connections = Vec::new();
        let (a, b) = start.pos;
        for i in [(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)] {
            if let Some(k) = my_map.get(&i) {
                if k.connections.contains(&start.pos) {
                    connections.push(i);
                }
            }
        }
        start.connections = (*connections).to_vec();
        let mut current = start.clone();
        let mut visited = Vec::new();
        loop {
            visited.push(current.pos);
            let connections = current.next(&my_map);
            let n: Vec<_> = connections
                .iter()
                .filter(|x| !visited.contains(&x.pos))
                .collect();

            if n.is_empty() {
                break;
            }
            current = n.first().unwrap().clone().clone();
        }
        (visited.len() / 2).to_string()
    }

    fn part2(&self, data: &str) -> String {
        let my_map = parse_b(data);

        let start: Vec<_> = my_map.values().filter(|x| x.starting).collect();
        let mut start = start.first().unwrap().clone().clone();
        let mut connections = Vec::new();
        let (a, b) = start.pos;
        for i in [(a - 1, b), (a + 1, b), (a, b - 1), (a, b + 1)] {
            if let Some(k) = my_map.get(&i) {
                if k.connections.contains(&start.pos) {
                    connections.push(i);
                }
            }
        }
        start.connections = (*connections).to_vec();
        let mut current = start.clone();
        let mut visited = Vec::new();
        loop {
            visited.push(current.pos);
            let connections = current.next(&my_map);
            let n: Vec<_> = connections
                .iter()
                .filter(|x| !visited.contains(&x.pos))
                .collect();

            if n.is_empty() {
                break;
            }
            current = n.first().unwrap().clone().clone();
        }
        visited.push(start.pos);
        let poly = geometry_rs::Polygon::new(
            visited
                .iter()
                .map(|coord| geometry_rs::Point {
                    x: coord.0 as f64,
                    y: coord.1 as f64,
                })
                .collect(),
            vec![],
        );
        let mut val = 0;
        for point in my_map.values() {
            if !visited.contains(&point.pos) {
                let p_out = geometry_rs::Point {
                    x: point.pos.0 as f64,
                    y: point.pos.1 as f64,
                };
                if poly.contains_point(p_out) {
                    val += 1;
                }
            }
        }

        val.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ex202310_a() {
        let data = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let d = D10 {};
        let res = d.part1(data);
        assert_eq!(res, "4");
    }
    #[test]
    fn test_ex202310_b() {
        let data = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let d = D10 {};
        let res = d.part2(data);
        assert_eq!(res, "4");
    }
    #[test]
    fn test_ex202310_b_2() {
        let data = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let d = D10 {};
        let res = d.part2(data);
        assert_eq!(res, "10");
    }
}
