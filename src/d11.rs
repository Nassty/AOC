use aoc_toolkit::Day;
pub struct D11 {}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn rotate_right(input: &str) -> String {
    let matrix: Vec<_> = input
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    transpose2(matrix)
        .iter()
        .map(|x| x.iter().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
fn rotate_left(input: &str) -> String {
    let matrix: Vec<_> = input
        .lines()
        .map(|row| row.chars().rev().collect::<Vec<_>>())
        .collect();
    transpose2(matrix)
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand(input: &str, factor: usize) -> String {
    let mut output = String::new();
    for line in input.lines() {
        output.push_str(line);
        output.push('\n');
        if line.chars().all(|x| x == '.') {
            for _ in 0..factor {
                output.push_str(line);
                output.push('\n');
            }
        }
    }
    output
}
fn parse_a(input: &str) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for (col, line) in input.lines().enumerate() {
        for (row, value) in line.chars().enumerate() {
            if value == '#' {
                res.push((row as i32, col as i32));
            }
        }
    }
    res
}
fn parse_b(input: &str) -> (Vec<(i32, i32)>, Vec<bool>, Vec<bool>) {
    let mut res = Vec::new();
    let mut rows_empty = Vec::new();
    let mut cols_empty = Vec::new();
    for (col, line) in input.lines().enumerate() {
        let mut col_empty = true;
        rows_empty.push(line.chars().all(|x| x == '*'));
        for (row, value) in line.chars().enumerate() {
            if value == '#' {
                col_empty = false;
                res.push((row as i32, col as i32));
            }
        }
        cols_empty.push(col_empty);
    }
    (res, rows_empty, cols_empty)
}
fn pre_process(input: &str, factor: usize) -> String {
    let k = expand(input, factor);
    let k = rotate_right(k.as_str());
    let k = expand(k.as_str(), factor);
    let k = rotate_left(k.as_str());
    k
}

fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

fn distance_cost(
    _p1: (i32, i32),
    _p2: (i32, i32),
    _cost: i32,
    _costs: (&Vec<bool>, &Vec<bool>),
) -> i32 {
    0
}

impl Day for D11 {
    fn part1(&self, data: &str) -> String {
        let parsed = parse_a(pre_process(data, 1).as_str());
        let mut acc = Vec::new();
        itertools::iproduct!(parsed.iter(), parsed.iter())
            .filter_map(|i| {
                let s = !acc.contains(&(i.1, i.0)) && i.0 != i.1;
                if s {
                    acc.push(i);
                    Some(i)
                } else {
                    None
                }
            })
            .map(|x| distance(*x.0, *x.1))
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, data: &str) -> String {
        let (parsed, rows_empty, cols_empty) = parse_b(data);
        let mut acc = Vec::new();
        itertools::iproduct!(parsed.iter(), parsed.iter())
            .filter_map(|i| {
                let s = !acc.contains(&(i.1, i.0)) && i.0 != i.1;
                if s {
                    acc.push(i);
                    Some(i)
                } else {
                    None
                }
            })
            .map(|x| distance_cost(*x.0, *x.1, 1_000_000, (&rows_empty, &cols_empty)))
            .sum::<i32>()
            .to_string()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex202311_a() {
        let data = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let d = D11 {};
        let res = d.part1(data);
        assert_eq!(res, "374");
    }
    #[test]
    fn test_ex202311_a_parse() {
        let data = "\
###
...
###";
        let expected = vec![(0, 0), (1, 0), (2, 0), (0, 2), (1, 2), (2, 2)];

        let res = parse_a(data);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_ex202311_both() {
        let data = "\
...
.#.
...";

        let expected = "\
.....
.....
..#..
.....
.....";

        let res = pre_process(data, 1);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_ex202311_expand() {
        let data = "\
...
.#.
...";
        let expected = "\
...
...
.#.
...
...
";
        let res = expand(data, 1);
        assert_eq!(res, expected);
    }
    #[test]
    fn test_rotate() {
        let input = "\
12
34";
        let output = "\
31
42";
        assert_eq!(rotate_right(input), output);
        assert_eq!(rotate_left(output), input);
    }
}
