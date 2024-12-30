use std::{env, fs};

fn look_and_say(input: &str, iterations: usize) -> String {
    let mut result = String::from(input);

    for _ in 0..iterations {
        let groups = group_digits(&result);

        result.clear();
        for group in groups {
            result.push_str(&group.len().to_string());
            result.push_str(&group[0].to_string());
        }
    }

    result
}

fn group_digits(input: &str) -> Vec<Vec<u8>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for ch in input.chars() {
        let digit = ch.to_digit(10).unwrap() as u8;

        if current_group.is_empty() || current_group[0] == digit {
            current_group.push(digit);
        } else {
            groups.push(current_group.clone());

            current_group.clear();
            current_group.push(digit);
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group.clone());
    }

    groups
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let input = input.trim();
    let result = look_and_say(&input, 50);

    println!("Look and Say: {}", result.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_groups_digits() {
        let input = "1";
        assert_eq!(group_digits(input), vec![vec![1]]);

        let input = "11";
        assert_eq!(group_digits(input), vec![vec![1, 1]]);

        let input = "21";
        assert_eq!(group_digits(input), vec![vec![2], vec![1]]);

        let input = "1211";
        assert_eq!(group_digits(input), vec![vec![1], vec![2], vec![1, 1]]);

        let input = "111221";
        assert_eq!(
            group_digits(input),
            vec![vec![1, 1, 1], vec![2, 2], vec![1]]
        );
    }

    #[test]
    fn it_generates() {
        let input = "1".to_string();
        let result = look_and_say(&input, 1);
        assert_eq!(result, "11".to_string());

        let input = "11".to_string();
        let result = look_and_say(&input, 1);
        assert_eq!(result, "21".to_string());

        let input = "21".to_string();
        let result = look_and_say(&input, 1);
        assert_eq!(result, "1211".to_string());

        let input = "1211".to_string();
        let result = look_and_say(&input, 1);
        assert_eq!(result, "111221".to_string());

        let input = "111221".to_string();
        let result = look_and_say(&input, 1);
        assert_eq!(result, "312211".to_string());

        let input = "1".to_string();
        let result = look_and_say(&input, 5);
        assert_eq!(result, "312211".to_string());
    }
}
