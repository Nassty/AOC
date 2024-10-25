use std::collections::HashMap;

use aoc_toolkit::{Day, Direction, Grid, Vec2};
pub struct D14 {}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Ball,
    Rock,
    Air,
}

impl D14 {
    fn parse(data: &str) -> Grid<Tile> {
        Grid::parse_char(data, &|x| match x {
            '.' => Tile::Air,
            '#' => Tile::Rock,
            'O' => Tile::Ball,
            _ => panic!("Invalid tile"),
        })
    }
    fn tilt(grid: &mut Grid<Tile>, dir: &Direction) {
        for _ in 0..grid.size().1 {
            for y in (0..grid.size().1).rev() {
                for x in (0..grid.size().0).rev() {
                    let pos = Vec2::new(x, y);
                    let target = pos.clone() + dir.delta().into();
                    if grid.get(&pos) == Some(Tile::Ball) && grid.get(&target) == Some(Tile::Air) {
                        grid.set(&pos, Tile::Air).unwrap();
                        grid.set(&target, Tile::Ball).unwrap();
                    }
                }
            }
        }
    }
    fn calculate(grid: &Grid<Tile>) -> usize {
        let total = grid.size().1;
        let mut resp = 0;
        for y in 0..total {
            let points = total - y;
            resp += points as usize
                * grid.get_tiles()[y as usize]
                    .iter()
                    .filter(|x| **x == Tile::Ball)
                    .count();
        }
        resp
    }
    fn display(grid: &Grid<Tile>) -> String {
        let mut res = String::new();
        let tiles = grid.get_tiles();
        (0..grid.size().1 as usize).for_each(|y| {
            for x in 0..grid.size().0 as usize {
                match tiles[y][x] {
                    Tile::Ball => res.push('O'),
                    Tile::Rock => res.push('#'),
                    Tile::Air => res.push('.'),
                }
            }
            res.push('\n');
        });
        res
    }
}

impl Day for D14 {
    fn part1(&self, data: &str) -> String {
        let mut grid = Self::parse(data);

        Self::tilt(&mut grid, &Direction::Up);
        Self::calculate(&grid).to_string()
    }

    fn part2(&self, data: &str) -> String {
        let total_cycles = 1_000_000_000;
        let res = {
            let mut cycle_size = 0;
            let mut grid = Self::parse(data);
            let mut hs: HashMap<String, usize> = HashMap::new();

            for index in 0..=1_000 {
                let digest = Self::display(&grid);
                if let Some(idx) = hs.get(&digest) {
                    cycle_size = index - idx;
                    dbg!(total_cycles % cycle_size);
                } else {
                    for dir in [
                        Direction::Up,
                        Direction::Left,
                        Direction::Down,
                        Direction::Right,
                    ] {
                        Self::tilt(&mut grid, &dir);
                    }
                    hs.insert(digest.clone(), index);
                }
            }
            cycle_size
        };

        let final_size = total_cycles % res;
        let mut grid = Self::parse(data);
        dbg!(final_size);
        dbg!(res);

        for _ in 0..final_size {
            for dir in [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ] {
                Self::tilt(&mut grid, &dir);
            }
        }
        Self::calculate(&grid).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .into()
    }

    #[test]
    fn test_part1() {
        let d = D14 {};
        assert_eq!("136", d.part1(&test_data()));
    }
    #[test]
    fn test_part2() {
        let d = D14 {};
        assert_eq!("64", d.part2(&test_data()));
    }
}
