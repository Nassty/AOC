use aoc_toolkit::Day;
pub struct D1 {}

impl Day for D1 {
    fn part1(&self, data: &str) -> String {
        data.split('\n')
            .filter(|k| !k.is_empty())
            .map(|x| {
                let p = x.chars().filter_map(|y| y.to_digit(10)).collect::<Vec<_>>();
                (p.first().unwrap().to_string() + &p.last().unwrap().to_string())
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, data: &str) -> String {
        let words: [(&str, u8); 9] = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        data.lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter_map(|(n, cchar)| {
                        if cchar.is_ascii_digit() {
                            return Some(cchar.to_string().parse::<u8>().unwrap());
                        }
                        let l = line[n..].to_string();
                        for (word, digit) in words {
                            if l.starts_with(word) {
                                return Some(digit);
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>()
            })
            .map(|x| {
                let s = x.clone();
                let p = s.first().unwrap();
                let k = x.last().unwrap();
                format!("{}{}", p, k).parse::<i32>().unwrap()
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D1 {};
        assert_eq!(
            "142",
            d.part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D1 {};
        assert_eq!(
            "281",
            d.part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            )
        );
    }
}
