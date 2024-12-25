use std::{env, fs};

fn solve(input: String) -> u32 {
    let mut length = 0;

    for line in input.lines() {
        let line = line.trim();
        let mut parts = line
            .split("x")
            .into_iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>();
        parts.sort_unstable();

        length += 2 * parts[0] + 2 * parts[1];
        length += parts[0] * parts[1] * parts[2];
    }

    length
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let length = solve(input);

    println!("Length: {length}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = "2x3x4".to_string();
        let length = solve(input);
        assert_eq!(length, 34);

        let input = "1x1x10".to_string();
        let length = solve(input);
        assert_eq!(length, 14);
    }
}
