use std::{
    cmp,
    collections::{HashMap, VecDeque},
};

use aoc_toolkit::Day;

#[derive(Debug)]
struct Line {
    base_origin: i64,
    base_destination: i64,
    offset: i64,
}
struct Limiter {
    lines: Vec<Line>,
}

impl Limiter {
    fn get(&self, input: i64) -> i64 {
        for line in &self.lines {
            if (line.base_origin..line.base_origin + line.offset).contains(&input) {
                return input + (line.base_destination - line.base_origin);
            }
        }
        input
    }
}
pub struct D5 {}

impl D5 {
    fn parse_seeds(input: &str) -> Vec<i64> {
        let binding = input.split("seeds: ").collect::<Vec<_>>();
        let values = binding.last().unwrap().split(" ");
        values
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect()
    }

    fn parse_seeds_2(input: &str) -> Vec<i64> {
        let data = Self::parse_seeds(input);
        let chunks = data.chunks(2).collect::<Vec<_>>();
        let mut output: Vec<i64> = Vec::new();
        for chunk in chunks {
            let first = chunk.first().unwrap();
            let last = chunk.last().unwrap();
            let range = *first..(first + last);
            let new_range: Vec<_> = range.collect();
            output.extend(new_range);
        }
        output
    }

    fn parse_data(data: Vec<&&str>) -> Limiter {
        let lines = Vec::new();
        let mut m = Limiter { lines };
        for line in data {
            let mut values = line
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let offset = values.pop().unwrap();
            let base_origin = values.pop().unwrap();
            let base_destination = values.pop().unwrap();
            m.lines.push(Line {
                offset,
                base_origin,
                base_destination,
            });
            assert_eq!(values.len(), 0);
        }
        m
    }
}

impl Day for D5 {
    fn part1(&self, data: &str) -> String {
        let mut sections = data.split("\n\n").collect::<VecDeque<_>>();
        let seeds = Self::parse_seeds(sections.pop_front().unwrap());
        let mut table = HashMap::new();
        for section in &sections {
            let lines = section.lines().collect::<Vec<_>>();
            let d = lines.iter().skip(1).collect::<Vec<_>>();
            let header = lines.first().unwrap().split(" ").collect::<Vec<_>>();
            let name = header.first().unwrap();
            table.insert(name.to_string(), Self::parse_data(d));
        }
        let path = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        let mut total = i64::MAX;
        for seed in seeds {
            let mut curr = seed;
            for point in path {
                curr = table[point].get(curr);
            }
            total = cmp::min(total, curr);
        }
        total.to_string()
    }

    fn part2(&self, data: &str) -> String {
        let mut sections = data.split("\n\n").collect::<VecDeque<_>>();
        let seeds = Self::parse_seeds_2(sections.pop_front().unwrap());
        let mut table = HashMap::new();
        for section in &sections {
            let lines = section.lines().collect::<Vec<_>>();
            let d = lines.iter().skip(1).collect::<Vec<_>>();
            let header = lines.first().unwrap().split(" ").collect::<Vec<_>>();
            let name = header.first().unwrap();
            table.insert(name.to_string(), Self::parse_data(d));
        }
        let path = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        let mut total = i64::MAX;
        for seed in seeds.iter() {
            let mut curr = *seed;
            for point in path {
                curr = table[point].get(curr);
            }
            total = cmp::min(total, curr);
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D5 {};
        assert_eq!(
            "35",
            d.part1(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D5 {};
        assert_eq!(
            "46",
            d.part2(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            )
        );
    }
}
