use std::{env, fs};

fn solve(input: &str) -> i32 {
    let mut result = 0;
    let mut buffer = String::new();

    for ch in input.chars().into_iter() {
        match ch {
            '-' => {
                if buffer.is_empty() {
                    buffer.push_str(&ch.to_string());
                }
            }
            ch if ch.is_digit(10) => {
                buffer.push_str(&ch.to_string());
            }
            _ => {
                if !buffer.is_empty() {
                    result += buffer.parse::<i32>().unwrap();
                    buffer.clear();
                }
            }
        }
    }

    result
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let result = solve(&input);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = "[1,2,3]".to_string();
        let result = solve(&input);
        assert_eq!(result, 6);

        let input = r#"{"a":2,"b":4}"#.to_string();
        let result = solve(&input);
        assert_eq!(result, 6);

        let input = "[[[3]]]".to_string();
        let result = solve(&input);
        assert_eq!(result, 3);

        let input = r#"{"a":{"b":4},"c":-1}"#.to_string();
        let result = solve(&input);
        assert_eq!(result, 3);

        let input = r#"{"a":[-1,1]}"#.to_string();
        let result = solve(&input);
        assert_eq!(result, 0);

        let input = r#"[-1,{"a":1}]"#.to_string();
        let result = solve(&input);
        assert_eq!(result, 0);

        let input = "[]".to_string();
        let result = solve(&input);
        assert_eq!(result, 0);

        let input = "{}".to_string();
        let result = solve(&input);
        assert_eq!(result, 0);
    }
}
