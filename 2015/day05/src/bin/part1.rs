use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Status {
    Nice,
    Naughty,
}

fn classify(text: &str) -> Status {
    let text = text.to_lowercase();

    // Check forbidden strings
    let forbidden = ["ab", "cd", "pq", "xy"];
    if forbidden.into_iter().any(|s| text.contains(s)) {
        return Status::Naughty;
    }

    // Check vowels
    let vowels = ["a", "e", "i", "o", "u"];
    if vowels
        .into_iter()
        .map(|v| text.match_indices(v).collect::<Vec<_>>().len())
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>()
        < 3
    {
        return Status::Naughty;
    }

    // Check double letters
    if !text.as_bytes().windows(2).any(|chars| chars[0] == chars[1]) {
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
        let input = "ugknbfddgicrmopn".to_string();
        assert_eq!(classify(&input), Status::Nice);

        let input = "aaa".to_string();
        assert_eq!(classify(&input), Status::Nice);

        let input = "jchzalrnumimnmhp".to_string();
        assert_eq!(classify(&input), Status::Naughty);

        let input = "haegwjzuvuyypxyu".to_string();
        assert_eq!(classify(&input), Status::Naughty);

        let input = "dvszwmarrgswjxmb".to_string();
        assert_eq!(classify(&input), Status::Naughty);
    }

    #[test]
    fn it_solves() {
        let input = "ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb"
            .to_string();
        let result = solve(&input);

        assert_eq!(result, 2);
    }
}
