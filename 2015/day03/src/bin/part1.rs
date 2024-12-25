use std::{collections::HashSet, env, fs};

fn solve(input: String) -> usize {
    let mut next = (0, 0);
    let mut houses = HashSet::new();
    houses.insert(next);

    for dir in input.trim().chars().into_iter() {
        match dir {
            '^' => next = (next.0, next.1 - 1),
            '>' => next = (next.0 + 1, next.1),
            'v' => next = (next.0, next.1 + 1),
            '<' => next = (next.0 - 1, next.1),
            _ => unreachable!(),
        }
        houses.insert(next);
    }

    houses.len()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let result = solve(input);

    println!("Number of houses that reveived gifts: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = ">".to_string();
        let result = solve(input);
        assert_eq!(result, 2);

        let input = "^>v<".to_string();
        let result = solve(input);
        assert_eq!(result, 4);

        let input = "^v^v^v^v^v".to_string();
        let result = solve(input);
        assert_eq!(result, 2);
    }
}
