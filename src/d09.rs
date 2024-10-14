use aoc_toolkit::Day;
pub struct D9 {}

impl D9 {
    fn parse_vec_sep(input: &str) -> Vec<i32> {
        input
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }

    fn difs(input: Vec<i32>) -> i32 {
        let mut total = *input.last().unwrap();
        let mut diffs: Vec<_> = input;
        while !diffs.iter().all(|x| *x == 0) {
            diffs = diffs
                .iter()
                .zip(diffs.iter().skip(1))
                .map(|k| k.1 - k.0)
                .collect();
            total += diffs.last().unwrap();
        }
        total
    }
}

impl Day for D9 {
    fn part1(&self, data: &str) -> String {
        data.lines()
            .map(Self::parse_vec_sep)
            .map(Self::difs)
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, data: &str) -> String {
        data.lines()
            .map(Self::parse_vec_sep)
            .map(|x| Self::difs(x.iter().copied().rev().collect()))
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D9 {};

        let data = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", d.part1(data));
    }
    #[test]
    fn test_part2() {
        let d = D9 {};
        let data = "10 13 16 21 30 45";
        assert_eq!("5", d.part2(data));
    }
}
