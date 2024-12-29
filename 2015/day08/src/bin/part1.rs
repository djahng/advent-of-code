use std::{env, fs};

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect::<Vec<_>>()
}

fn solve(lines: &[&str]) -> usize {
    let mut n_code = 0;
    let mut n_char = 0;

    for line in lines {
        let line = line.trim();
        n_code += line.chars().count();

        let mut idx = 0;
        while idx < line.len() {
            match line.chars().nth(idx).unwrap() {
                '"' => {
                    idx += 1;
                    continue;
                }
                '\\' => {
                    if let Some(next_ch) = line.chars().nth(idx + 1) {
                        match next_ch {
                            '\\' => {
                                n_char += 1;
                                idx += 2;
                            }
                            '"' => {
                                n_char += 1;
                                idx += 2;
                            }
                            'x' => {
                                n_char += 1;
                                idx += 4;
                            }
                            _ => {
                                idx += 1;
                            }
                        }
                    }
                }
                _ => {
                    n_char += 1;
                    idx += 1;
                }
            }
        }
    }

    n_code - n_char
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let lines = parse(&input);
    let result = solve(&lines);

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
            .to_string();
        let lines = parse(&input);

        assert_eq!(
            lines,
            vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#]
        );
    }

    #[test]
    fn it_solves() {
        let input = r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
            .to_string();
        let lines = parse(&input);
        let result = solve(&lines);

        assert_eq!(result, 12);

        let input = r#""byc\x9dyxuafof\\\xa6uf\"axfozomj\\olh\x6a""#.to_string();
        let lines = parse(&input);
        let result = solve(&lines);

        assert_eq!(result, 43 - 29);
    }
}
