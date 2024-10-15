use aoc_toolkit::{Day, Grid};
use std::iter::zip;
pub struct D13 {}

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

impl D13 {
    fn parse(input: &str) -> Vec<Grid<Tile>> {
        input
            .split("\n\n")
            .map(|t| {
                Grid::parse_char(t, &|x: char| match x {
                    '.' => Tile::Ash,
                    '#' => Tile::Rock,
                    _ => panic!("Invalid tile"),
                })
            })
            .collect()
    }

    fn compute(&self, grid: &Grid<Tile>) -> Option<usize> {
        let size = grid.size();
        for index in 1..size.1 {
            let index = index as usize;
            let before = grid.get_tiles()[..index].iter();
            let after = grid.get_tiles()[index..].iter();
            let min = before.len().min(after.len());
            let before = before.rev().take(min).rev();
            let after = after.take(min).rev();
            let k = after.clone().collect::<Vec<_>>();
            if zip(before, after).all(|(a, b)| a == b) && !k.is_empty() {
                return Some(index);
            }
        }
        None
    }

    fn compute2(&self, grid: &Grid<Tile>) -> Option<usize> {
        None
    }
    fn solve(&self, grids: &[Grid<Tile>], f: impl Fn(&Grid<Tile>) -> Option<usize>) -> usize {
        grids
            .iter()
            .map(|grid| {
                f(grid)
                    .map(|x| x * 100)
                    .or_else(|| f(&grid.transpose()))
                    .expect("one solution")
            })
            .sum::<usize>()
    }
}

impl Day for D13 {
    fn part1(&self, data: &str) -> String {
        self.solve(&Self::parse(data), |x| self.compute(x))
            .to_string()
    }

    fn part2(&self, data: &str) -> String {
        self.solve(&Self::parse(data), |x| self.compute2(x))
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let d = D13 {};
        let res = d.part1(data);
        assert_eq!(res, "405");
    }
    #[test]
    fn test_part2() {
        let data = "\
        #.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let d = D13 {};
        let res = d.part2(data);
        assert_eq!(res, "400");
    }
}
