use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Status {
    Nice,
    Naughty,
}

fn classify(text: &str) -> Status {
    let text = text.to_lowercase();

    // Check pairs of letters that appear at least twice without overlapping
    let mut pairs = false;
    for i in 0..text.len() - 2 {
        for j in i + 2..text.len() - 1 {
            if (text.chars().nth(i).unwrap() == text.chars().nth(j).unwrap())
                && (text.chars().nth(i + 1).unwrap() == text.chars().nth(j + 1).unwrap())
            {
                pairs = true;
                break;
            }
        }
    }
    if !pairs {
        return Status::Naughty;
    }

    // Check letters that repeat with a different letter between them
    if !text
        .as_bytes()
        .windows(3)
        .any(|chars| (chars[0] == chars[2]) && (chars[0] != chars[1]))
    {
        return Status::Naughty;
    }

    Status::Nice
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|&text| classify(text.trim()) == Status::Nice)
        .count()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let n_nice = solve(&input);

    println!("Number of nice strings: {n_nice}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_classifies() {
        let input = "qjhvhtzxzqqjkmpb".to_string();
        assert_eq!(classify(&input), Status::Nice);

        let input = "xxyxx".to_string();
        assert_eq!(classify(&input), Status::Nice);

        let input = "uurcxstgmygtbstg".to_string();
        assert_eq!(classify(&input), Status::Naughty);

        let input = "ieodomkazucvgmuy".to_string();
        assert_eq!(classify(&input), Status::Naughty);
    }

    #[test]
    fn it_solves() {
        let input = "qjhvhtzxzqqjkmpb
        xxyxx
        uurcxstgmygtbstg
        ieodomkazucvgmuy"
            .to_string();
        let result = solve(&input);

        assert_eq!(result, 2);
    }
}
