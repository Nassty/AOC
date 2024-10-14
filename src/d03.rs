use aoc_toolkit::Day;
pub struct D3 {}

#[derive(Debug, Clone)]
struct Piece<'a> {
    line: i32,
    col: i32,
    value: &'a str,
}

#[derive(Debug)]
struct Symbol {
    line: i32,
    pos: i32,
}

impl Day for D3 {
    fn part1(&self, data: &str) -> String {
        let lines = data.lines();
        let mut symbols = Vec::new();
        let mut pieces = Vec::new();

        for (line_no, line) in lines.clone().enumerate() {
            let chars: Vec<_> = line.chars().collect();
            let mut col_no = 0;
            while col_no < line.len() {
                let cchar = chars[col_no];
                match cchar {
                    '.' => {
                        col_no += 1;
                    }
                    '0'..='9' => {
                        let start = col_no;
                        while col_no < line.len() && chars[col_no].is_ascii_digit() {
                            col_no += 1;
                        }

                        pieces.push(Piece {
                            line: line_no as i32,
                            col: start as i32,
                            value: &line[start..col_no],
                        });
                    }
                    _ => {
                        symbols.push(Symbol {
                            line: line_no as i32,
                            pos: col_no as i32,
                        });
                        col_no += 1;
                    }
                }
            }
        }
        let ls = lines.collect::<Vec<_>>();
        let mut total_pieces: Vec<&Piece> = Vec::new();
        for symbol in &symbols {
            for line_offset in [-1, 0, 1] {
                if symbol.line < 0 || symbol.line > ls.len() as i32 {
                    continue;
                }
                let current_line_no = symbol.line + line_offset;
                let filtered = pieces
                    .iter()
                    .filter(|piece| {
                        piece.line == current_line_no
                            && ((piece.col - 1)..=piece.col + piece.value.len() as i32)
                                .contains(&symbol.pos)
                    })
                    .collect::<Vec<_>>();
                total_pieces.extend(filtered.clone());
            }
        }
        let mut processed: Vec<(i32, i32)> = Vec::new();
        let mut total = 0;

        for piece in total_pieces {
            let id = (piece.line, piece.col);
            if !processed.contains(&id) {
                processed.push(id);
                total += piece.value.parse::<i32>().unwrap();
            }
        }
        total.to_string()
    }

    fn part2(&self, data: &str) -> String {
        let lines = data.lines();
        let mut symbols = Vec::new();
        let mut pieces = Vec::new();

        for (line_no, line) in lines.clone().enumerate() {
            let chars: Vec<_> = line.chars().collect();
            let mut col_no = 0;
            while col_no < line.len() {
                let cchar = chars[col_no];
                match cchar {
                    '0'..='9' => {
                        let start = col_no;
                        while col_no < line.len() && chars[col_no].is_ascii_digit() {
                            col_no += 1;
                        }

                        pieces.push(Piece {
                            line: line_no as i32,
                            col: start as i32,
                            value: &line[start..col_no],
                        });
                    }
                    '*' => {
                        symbols.push(Symbol {
                            line: line_no as i32,
                            pos: col_no as i32,
                        });
                        col_no += 1;
                    }
                    _ => {
                        col_no += 1;
                    }
                }
            }
        }
        let ls = lines.collect::<Vec<_>>();
        let mut total = 0;
        for symbol in &symbols {
            let mut total_pieces: Vec<&Piece> = Vec::new();
            for line_offset in [-1, 0, 1] {
                if symbol.line < 0 || symbol.line > ls.len() as i32 {
                    continue;
                }
                let current_line_no = symbol.line + line_offset;
                let filtered = pieces
                    .iter()
                    .filter(|piece| {
                        piece.line == current_line_no
                            && ((piece.col - 1)..=piece.col + piece.value.len() as i32)
                                .contains(&symbol.pos)
                    })
                    .collect::<Vec<_>>();
                total_pieces.extend(filtered.clone());
            }

            if total_pieces.len() > 1 {
                let mut partial = 1;
                for piece in total_pieces {
                    partial *= piece.value.parse::<i32>().unwrap();
                }
                total += partial;
            }
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31_1() {
        let data = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let d = D3 {};
        let res = d.part1(data);
        assert_eq!(res, "4361");
    }
    #[test]
    fn test_31_2() {
        let data = "\
123......123
*........123";
        let d = D3 {};
        let res = d.part1(data);
        assert_eq!(res, "123");
    }
    #[test]
    fn test_31_3() {
        let data = "\
123......123
..........*.
.........123";
        let d = D3 {};
        let res = d.part1(data);
        assert_eq!(res, "246");
    }
    #[test]
    fn test_31_4() {
        let data = "\
123......123
..........**
.........123";
        let d = D3 {};
        let res = d.part1(data);
        assert_eq!(res, "246");
    }
    #[test]
    fn test_31_5() {
        let data = "\
1.1.1.1.1
#########
.........
";
        let d = D3 {};
        let res = d.part1(data);
        assert_eq!(res, "5");
    }
    #[test]
    fn test_32_1() {
        let data = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let d = D3 {};
        let res = d.part2(data);
        assert_eq!(res, "467835");
    }
}
