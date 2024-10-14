use aoc_toolkit::Day;
use cached::proc_macro::cached;

pub struct D12 {}

#[derive(Debug, PartialEq)]
struct Line {
    src: String,
    damaged: Vec<i64>,
}

impl Line {
    fn new(src: String, damaged: &str, scale: usize) -> Self {
        let src = vec![src.clone(); scale].join("?");
        let d: Vec<_> = damaged
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .cycle()
            .take(damaged.split(",").collect::<Vec<_>>().len() * scale)
            .collect();

        Self { src, damaged: d }
    }
    fn solve(&self) -> i64 {
        count_arrangements(self.src.to_string(), self.damaged.clone())
    }
}
#[cached]
fn count_arrangements(record: String, damaged_count: Vec<i64>) -> i64 {
    if record.is_empty() {
        if !damaged_count.is_empty() {
            return 0;
        }
        return 1;
    }
    if damaged_count.is_empty() {
        if record.contains("#") {
            return 0;
        }
        return 1;
    }
    let fst = record.chars().next().unwrap();
    match fst {
        '.' => {
            return count_arrangements(record[1..].to_string(), damaged_count);
        }
        '?' => {
            let k = count_arrangements(format!(".{}", &record[1..]), damaged_count.clone())
                + count_arrangements(format!("#{}", &record[1..]), damaged_count.clone());
            return k;
        }
        _ => {}
    }
    let n = damaged_count[0] as usize;
    let c = record.as_str();
    if record.len() < n
        || c[..n].contains('.')
        || (record.len() > n && c.chars().nth(n).unwrap() == '#')
    {
        return 0;
    }
    if c.len() <= n {
        return count_arrangements(String::new(), damaged_count[1..].to_vec());
    }
    return count_arrangements(c[n + 1..].to_string(), damaged_count[1..].to_vec());
}

fn parse_line(input: &str, scale: usize) -> Line {
    let (elements, inputs) = input.split_once(" ").unwrap();
    Line::new(elements.to_string(), inputs, scale)
}

fn parse_a(input: &str) -> Vec<Line> {
    input.lines().map(|x| parse_line(x, 1)).collect()
}

fn parse_b(input: &str) -> Vec<Line> {
    input.lines().map(|x| parse_line(x, 5)).collect()
}

impl Day for D12 {
    fn part1(&self, data: &str) -> String {
        let parsed = parse_a(data);
        parsed.iter().map(|x| x.solve()).sum::<i64>().to_string()
    }

    fn part2(&self, data: &str) -> String {
        let parsed = parse_b(data);
        parsed.iter().map(|x| x.solve()).sum::<i64>().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ex202312_a() {
        let data = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let d = D12 {};
        let res = d.part1(data);
        assert_eq!(res, "21");
    }
    #[test]
    fn test_ex202312_b() {
        let data = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        // let data = ".??..??...?##. 1,1,3";
        let d = D12 {};
        let res = d.part2(data);
        assert_eq!(res, "525152");
    }
}
