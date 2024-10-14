#[cfg(feature = "draw")]
use raylib::prelude::*;

#[cfg(not(feature = "draw"))]
use indicatif::ParallelProgressIterator;
#[cfg(not(feature = "draw"))]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use aoc_toolkit::Day;
pub struct D16 {}

#[derive(Debug, Clone)]
enum Tile {
    Horizontal,
    Vertical,
    Backslash,
    Slash,
    Space,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Tile::Horizontal => "-",
            Tile::Vertical => "|",
            Tile::Backslash => "\\",
            Tile::Slash => "/",
            Tile::Space => ".",
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Debug)]
struct Cursor {
    x: usize,
    y: usize,
    map_x: usize,
    map_y: usize,
    heading: Direction,
}

impl Cursor {
    fn new(x: usize, y: usize, map_x: usize, map_y: usize, heading: Direction) -> Self {
        Self {
            x,
            y,
            map_x,
            map_y,
            heading,
        }
    }

    fn rot(self, tile: &Tile) -> Vec<Self> {
        let heading = match (self.heading, tile) {
            (Direction::Right, Tile::Backslash) => Direction::Down,
            (Direction::Right, Tile::Slash) => Direction::Up,
            (Direction::Left, Tile::Backslash) => Direction::Up,
            (Direction::Left, Tile::Slash) => Direction::Down,
            (Direction::Up, Tile::Backslash) => Direction::Left,
            (Direction::Up, Tile::Slash) => Direction::Right,
            (Direction::Down, Tile::Backslash) => Direction::Right,
            (Direction::Down, Tile::Slash) => Direction::Left,
            _ => unreachable!("OOOOOPSSS"),
        };
        Self::new(self.x, self.y, self.map_x, self.map_y, heading).step()
    }

    fn step(&self) -> Vec<Self> {
        let new_pos = match self.heading {
            Direction::Right => {
                if self.x >= self.map_x {
                    None
                } else {
                    Some((self.x + 1, self.y))
                }
            }
            Direction::Left => {
                if self.x < 1 {
                    None
                } else {
                    Some((self.x - 1, self.y))
                }
            }
            Direction::Up => {
                if self.y < 1 {
                    None
                } else {
                    Some((self.x, self.y - 1))
                }
            }
            Direction::Down => {
                if self.y >= self.map_y {
                    None
                } else {
                    Some((self.x, self.y + 1))
                }
            }
        };
        if let Some((x, y)) = new_pos {
            vec![Self::new(
                x,
                y,
                self.map_x,
                self.map_y,
                self.heading.clone(),
            )]
        } else {
            vec![]
        }
    }
    fn split(self, tile: &Tile) -> Vec<Self> {
        let heading = self.heading.clone();
        match (heading, tile) {
            (Direction::Right | Direction::Left, Tile::Horizontal) => self.step(),
            (Direction::Up | Direction::Down, Tile::Vertical) => self.step(),
            (Direction::Right | Direction::Left, Tile::Vertical) => {
                let mut u = self.clone();
                let mut d = self.clone();
                u.heading = Direction::Up;
                d.heading = Direction::Down;
                let mut rs = d.step();
                rs.append(&mut u.step());
                rs
            }
            (Direction::Up | Direction::Down, Tile::Horizontal) => {
                let mut l = self.clone();
                let mut r = self.clone();
                l.heading = Direction::Left;
                r.heading = Direction::Right;
                let mut rs = r.step();
                rs.append(&mut l.step());
                rs
            }
            _ => unreachable!(),
        }
    }

    fn advance(self, tile: &Tile) -> Vec<Self> {
        match tile {
            Tile::Horizontal | Tile::Vertical => self.split(tile),
            Tile::Backslash | Tile::Slash => self.rot(tile),
            _ => self.step(),
        }
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-" => Self::Horizontal,
            "|" => Self::Vertical,
            "\\" => Self::Backslash,
            "/" => Self::Slash,
            "." => Self::Space,
            _ => return Err(format!("can't parse {s}")),
        })
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|s| s.to_string().as_str().parse::<Tile>().expect("tile"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
fn solve(grid: Vec<Vec<Tile>>, cursor: Cursor) -> usize {
    let x = grid[0].len();
    let y = grid.len();
    let x1 = cursor.x;
    let y1 = cursor.y;
    let mut hs = HashSet::from([(x1, y1)]);
    let mut hist = HashSet::from([(x1, y1, cursor.heading.clone())]);
    let mut queue = cursor.advance(&grid[x1][y1]);
    let mut current_score = 0;
    let mut iterations = 0;

    #[cfg(feature = "draw")]
    let size = 5;
    #[cfg(feature = "draw")]
    let (mut rl, thread) = raylib::init()
        .size(x as i32 * size, y as i32 * size)
        .title("Hello, World")
        .build();
    while !queue.is_empty() {
        #[cfg(feature = "draw")]
        let mut d = rl.begin_drawing(&thread);
        #[cfg(feature = "draw")]
        d.clear_background(Color::BLACK);
        for q in &queue {
            if q.x < x && q.y < y {
                hs.insert((q.x, q.y));
            }
        }
        if hs.len() == current_score {
            iterations += 1;
            if iterations == 500 {
                break;
            }
        }
        if hs.len() != current_score {
            iterations = 0;
        }
        queue = queue
            .iter()
            .cloned()
            .flat_map(|current| {
                let (x, y) = (current.x, current.y);
                if x >= current.map_x || y >= current.map_y {
                    return vec![];
                }
                if hist.contains(&(current.x, current.y, current.heading.clone())) {
                    return vec![];
                }
                hist.insert((current.x, current.y, current.heading.clone()));

                current.advance(&grid[y][x])
            })
            .collect::<Vec<_>>();
        current_score = hs.len();
        #[cfg(feature = "draw")]
        #[cfg(feature = "draw")]
        for (y, line) in grid.iter().enumerate() {
            for (x, col) in line.iter().enumerate() {
                let draw_x = x as i32 * size;
                let draw_y = y as i32 * size;
                if hs.contains(&(x, y)) {
                    d.draw_rectangle(draw_x, draw_y, size, size, Color::RED);
                }
                d.draw_text(
                    format!("{col}").as_str(),
                    draw_x,
                    draw_y + size / 2,
                    size,
                    Color::WHITE,
                );
                for q in &queue {
                    let draw_x = q.x as i32 * size;
                    let draw_y = q.y as i32 * size;
                    d.draw_rectangle(draw_x, draw_y, size, size, Color::BLUE);
                }
                d.draw_text(format!("{}", queue.len()).as_str(), 0, 0, 40, Color::YELLOW);
            }
        }
        //println!("--------- END {} -------", hs.len());
        //let _ = io::stdin().read_line(&mut String::new());
    }
    hs.len()
}
fn solve1(input: &str) -> usize {
    let grid = parse_grid(input);
    let x = grid[0].len();
    let y = grid.len();
    let cursor = Cursor::new(0, 0, x, y, Direction::Right);
    solve(grid, cursor)
}

fn solve2(input: &str) -> usize {
    let grid = parse_grid(input);
    let x = grid[0].len();
    let y = grid.len();
    let mut cases = vec![];
    for y1 in 0..y - 1 {
        cases.push(Cursor::new(0, y1, x, y, Direction::Right));
        cases.push(Cursor::new(x - 1, y1, x, y, Direction::Left));
    }
    for x1 in 0..x - 1 {
        cases.push(Cursor::new(x1, 0, x, y, Direction::Down));
        cases.push(Cursor::new(x1, y - 1, x, y, Direction::Up));
    }
    #[cfg(not(feature = "draw"))]
    let v = cases.len();

    rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build_global()
        .unwrap();

    #[cfg(feature = "draw")]
    return cases
        .iter()
        .cloned()
        .map(|c| solve(grid.clone(), c))
        .max()
        .expect("number");
    #[cfg(not(feature = "draw"))]
    cases
        .par_iter()
        .cloned()
        .progress_count(v as u64)
        .map(|c| solve(grid.clone(), c))
        .max()
        .expect("number")
}

impl Day for D16 {
    fn part1(&self, input: &str) -> String {
        solve1(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        solve2(input).to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(
            8,
            solve1(
                r"--\
|.|
\-/"
            )
        );
    }
    #[test]
    fn test_parse() {
        assert_eq!(
            46,
            solve1(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
            )
        );
    }
    #[test]
    fn test_parse2() {
        assert_eq!(
            51,
            solve2(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
            )
        );
    }
    #[test]
    fn test_solve2() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let grid = parse_grid(input);
        let x = grid[0].len();
        let y = grid.len();
        let cursor = Cursor::new(3, 0, x, y, Direction::Down);
        assert_eq!(51, solve(grid, cursor));
    }
}
