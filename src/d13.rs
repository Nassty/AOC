use aoc_toolkit::Day;
pub struct D13 {}
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    Iterator::collect((0..len).map(|_| {
        iters
            .iter_mut()
            .map(|n| n.next().unwrap())
            .collect::<Vec<T>>()
    }))
}

fn rotate_left(input: &str) -> String {
    transpose2(
        input
            .lines()
            .map(|row| row.chars().rev().collect::<Vec<_>>())
            .collect(),
    )
    .iter()
    .map(|x| x.iter().collect::<String>())
    .collect::<Vec<_>>()
    .join("\n")
}

fn parse_a(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn parse_b(input: &str) -> &str {
    input
}

fn check_mirror(lhs: &str, rhs: &str) -> bool {
    if lhs.len() <= 1 || rhs.len() <= 1 {
        return false;
    }
    if lhs == rhs.chars().rev().collect::<String>().as_str() {
        return true;
    }

    if lhs.len() < rhs.len() {
        return check_mirror(lhs, &rhs[lhs.len()..]);
    }

    if lhs.len() > rhs.len() {
        return check_mirror(&lhs[rhs.len()..], rhs);
    }
    check_mirror(&lhs[1..], &rhs[1..])
}

fn get_mirrors_on(block: &str) -> Vec<usize> {
    let mut candidates: Vec<_> = (1..block.lines().next().unwrap().len()).collect();
    for line in block.lines() {
        let mut new_candidates = Vec::new();
        for index in candidates.clone() {
            if check_mirror(&line[..index - 1], &line[index..]) {
                new_candidates.push(index);
            }
            candidates = new_candidates.clone();
        }
    }
    candidates
}

impl Day for D13 {
    fn part1(&self, data: &str) -> String {
        let parsed = parse_a(data);
        let mut total_rows: Vec<usize> = Vec::new();
        let mut total_cols: Vec<usize> = Vec::new();
        for block in parsed {
            let rows = get_mirrors_on(block);
            total_rows.extend(rows.iter());
            let cols = get_mirrors_on(rotate_left(block).as_str());
            total_cols.extend(cols.iter());
        }

        (total_rows.iter().sum::<usize>() + total_cols.iter().map(|x| (x) * 100).sum::<usize>())
            .to_string()
    }

    fn part2(&self, data: &str) -> String {
        let _parsed = parse_b(data);
        0.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex202313_a() {
        let data = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let d = D13 {};
        let res = d.part1(data);
        assert_eq!(res, "405");
    }
}
