use aoc_toolkit::Day;
pub struct D4 {}

impl D4 {
    fn steps(mut input: Vec<(i32, i32)>) -> i32 {
        let mut total = 0;
        let mut c = 0;
        while c < input.len() {
            let original = input[c];

            let copias = original.0;
            let winners = original.1;
            let mut idx_0 = 1;
            while idx_0 <= copias {
                let mut next = 1;
                while next <= winners {
                    input[c + next as usize].0 += 1;
                    next += 1;
                }
                idx_0 += 1;
            }
            c += 1;
            total += original.0;
        }
        total
    }

    fn parse_vec_sep(input: &str, sep: &str) -> Vec<i32> {
        input
            .split(sep)
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }
}

impl Day for D4 {
    fn part1(&self, data: &str) -> String {
        let lines = data.lines();
        let mut total = 0;
        for line in lines {
            let mut val = 0;
            let rest = line.split(": ").last().unwrap();
            let numbers = rest.split(" | ").collect::<Vec<_>>();
            let current = Self::parse_vec_sep(numbers.first().unwrap(), " ");
            let winning = Self::parse_vec_sep(numbers.last().unwrap(), " ");
            for win in winning {
                if current.contains(&win) {
                    val += 1;
                }
            }
            if val >= 1 {
                total += 2_i32.pow(val - 1);
            }
        }
        total.to_string()
    }
    fn part2(&self, data: &str) -> String {
        let lines = data.lines();
        let mut cards = Vec::new();
        for line in lines {
            let mut val = 0;
            let rest = line.split(": ").last().unwrap();
            let numbers = rest.split(" | ").collect::<Vec<_>>();
            let current = Self::parse_vec_sep(numbers.first().unwrap(), " ");
            let winning = Self::parse_vec_sep(numbers.last().unwrap(), " ");
            for win in winning {
                if current.contains(&win) {
                    val += 1;
                }
            }
            cards.push((1, val));
        }

        Self::steps(cards).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D4 {};
        assert_eq!(
            "13",
            d.part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D4 {};
        assert_eq!(
            "30",
            d.part2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
        );
    }
}
