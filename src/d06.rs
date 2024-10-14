use aoc_toolkit::Day;
pub struct D6 {}

impl Day for D6 {
    fn part1(&self, data: &str) -> String {
        let mut values = Vec::new();
        for line in data.lines().filter(|x| !x.is_empty()) {
            let binding = line.split(":");
            let val = binding.last().unwrap();
            let mut i = 0;
            let mut cval = Vec::new();
            let chars: Vec<_> = val.chars().collect();
            while i < chars.len() {
                match chars[i] {
                    '0'..='9' => {
                        let start = i;
                        while i < chars.len() && chars[i].is_ascii_digit() {
                            i += 1;
                        }
                        cval.push(val[start..i].parse::<i32>().unwrap());
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            values.push(cval);
        }
        let head = values.first().unwrap();
        let tail = values.get(1).unwrap();
        let v: Vec<_> = head.iter().zip(tail.iter()).collect();
        let mut totals = Vec::new();
        for (time, distance) in &v {
            let mut partials = 0;
            for i in 0..**time {
                let speed = i;
                let rest = *time - i;
                if speed * rest > **distance {
                    partials += 1;
                }
            }
            totals.push(partials);
        }
        let mut base = 1;
        for t in totals {
            base *= t;
        }
        base.to_string()
    }

    fn part2(&self, data: &str) -> String {
        let values = data
            .lines()
            .map(|x| {
                x.split(":")
                    .last()
                    .unwrap()
                    .trim()
                    .replace(" ", "")
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let (time, distance) = (values[0], values[1]);
        let mut partials = 0;
        for i in 0..time {
            let speed = i;
            let rest = time - i;
            if speed * rest > distance {
                partials += 1;
            }
        }
        partials.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let d = D6 {};
        assert_eq!(
            "288",
            d.part1(
                "Time:      7  15   30
Distance:  9  40  200"
            )
        );
    }
    #[test]
    fn test_part2() {
        let d = D6 {};
        assert_eq!(
            "71503",
            d.part2(
                "Time:      7  15   30
Distance:  9  40  200"
            )
        );
    }
}
