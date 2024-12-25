use std::{collections::HashSet, env, fs};

fn solve(input: String) -> usize {
    let mut santa_next = (0, 0);
    let mut robo_santa_next = (0, 0);
    let mut santa_houses = HashSet::new();
    let mut robo_santa_houses = HashSet::new();
    santa_houses.insert(santa_next);
    robo_santa_houses.insert(robo_santa_next);

    for (i, dir) in input.trim().chars().into_iter().enumerate() {
        if i % 2 == 0 {
            match dir {
                '^' => santa_next = (santa_next.0, santa_next.1 - 1),
                '>' => santa_next = (santa_next.0 + 1, santa_next.1),
                'v' => santa_next = (santa_next.0, santa_next.1 + 1),
                '<' => santa_next = (santa_next.0 - 1, santa_next.1),
                _ => unreachable!(),
            }
            santa_houses.insert(santa_next);
        } else {
            match dir {
                '^' => robo_santa_next = (robo_santa_next.0, robo_santa_next.1 - 1),
                '>' => robo_santa_next = (robo_santa_next.0 + 1, robo_santa_next.1),
                'v' => robo_santa_next = (robo_santa_next.0, robo_santa_next.1 + 1),
                '<' => robo_santa_next = (robo_santa_next.0 - 1, robo_santa_next.1),
                _ => unreachable!(),
            }
            robo_santa_houses.insert(robo_santa_next);
        }
    }

    santa_houses.extend(robo_santa_houses);
    santa_houses.len()
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
        let input = "^v".to_string();
        let result = solve(input);
        assert_eq!(result, 3);

        let input = "^>v<".to_string();
        let result = solve(input);
        assert_eq!(result, 3);

        let input = "^v^v^v^v^v".to_string();
        let result = solve(input);
        assert_eq!(result, 11);
    }
}
