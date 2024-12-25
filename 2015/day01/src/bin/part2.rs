use std::{env, fs};

fn parse(input: &str) -> Option<usize> {
    let mut floor = 0;

    for (idx, ch) in input.trim().chars().into_iter().enumerate() {
        match ch {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => unreachable!(),
        }

        if floor == -1 {
            return Some(idx + 1);
        }
    }

    None
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(path).expect("to read file");

    if let Some(position) = parse(&input) {
        println!("-1 Position: {position}");
    } else {
        println!("Floor -1 never reached!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = ")".to_string();
        let floor = parse(&input);
        assert_eq!(floor, Some(1));

        let input = "()())".to_string();
        let floor = parse(&input);
        assert_eq!(floor, Some(5));
    }
}
