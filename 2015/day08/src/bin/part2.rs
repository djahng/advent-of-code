use std::{env, fs};

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect::<Vec<_>>()
}

fn solve(lines: &[&str]) -> usize {
    let mut n_char = 0;
    let n_code = lines
        .iter()
        .map(|line| line.trim().chars().count())
        .sum::<usize>();

    for line in lines {
        let line = line.trim();

        let mut idx = 0;
        while idx < line.len() {
            match line.chars().nth(idx).unwrap() {
                '"' => {
                    // " becomes \"
                    n_char += 2;
                    idx += 1;
                }
                '\\' => {
                    n_char += 2;
                    idx += 1;
                }
                _ => {
                    n_char += 1;
                    idx += 1;
                }
            }
        }

        // Add the first and last double quote back in
        n_char += 2;
    }

    n_char - n_code
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

        assert_eq!(result, 19);

        let input = r#""b\x9f\\\xa6f\"j\\o\x6""#.to_string();
        let lines = parse(&input);
        let result = solve(&lines);

        assert_eq!(result, 36 - 23);
    }
}
