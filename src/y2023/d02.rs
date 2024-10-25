use std::cmp;

use aoc_toolkit::Day;

struct Game<'a> {
    plays: &'a mut Vec<Play>,
}
#[derive(Debug)]
struct Play {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

impl<'a> Game<'a> {
    fn feed_line(&mut self, line: &str) {
        let mut p = Play {
            id: 0,
            blue: 0,
            red: 0,
            green: 0,
        };
        let info = line.split(":").collect::<Vec<_>>();
        p.id = info[0].split("Game ").collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();
        let sets = info[1].split(";").collect::<Vec<_>>();
        for set in sets {
            for color in set.split(',') {
                let r = color.split(" ").filter(|x| x != &"").collect::<Vec<_>>();
                let val = r[0].parse::<u32>().unwrap();
                let name = r[1];
                match name {
                    "blue" => p.blue = cmp::max(p.blue, val),
                    "red" => p.red = cmp::max(p.red, val),
                    "green" => p.green = cmp::max(p.green, val),
                    _ => todo!(),
                }
            }
        }
        self.plays.push(p);
    }

    fn find_pows(&mut self) -> u32 {
        let mut total = 0;
        for play in &mut *self.plays {
            total += play.red * play.green * play.blue;
        }
        total
    }

    fn find_constrains(&mut self, red: u32, green: u32, blue: u32) -> u32 {
        let mut final_value = 0;
        for play in &mut *self.plays {
            if !(play.red > red || play.blue > blue || play.green > green) {
                final_value += play.id
            }
        }
        final_value
    }
}

pub struct D2 {}

impl Day for D2 {
    fn part1(&self, data: &str) -> String {
        let lines = data.lines();
        let mut plays = Vec::new();
        let mut g = Game { plays: &mut plays };
        for line in lines {
            g.feed_line(line)
        }
        g.find_constrains(12, 13, 14).to_string()
    }

    fn part2(&self, data: &str) -> String {
        let lines = data.lines();
        let mut plays = Vec::new();
        let mut g = Game { plays: &mut plays };
        for line in lines {
            g.feed_line(line)
        }
        g.find_pows().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let d = D2 {};
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(d.part1(input), "8");
    }
    #[test]
    fn test_part2() {
        let d = D2 {};
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(d.part2(input), "2286");
    }
}
