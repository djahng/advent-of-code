use std::{env, fs};

fn parse(input: &str) -> i32 {
    let mut floor = 0;

    for ch in input.trim().chars().into_iter() {
        match ch {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => unreachable!(),
        }
    }

    floor
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let floor = parse(&input);

    println!("Floor: {floor}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = "(())".to_string();
        let floor = parse(&input);
        assert_eq!(floor, 0);

        let input = "()()".to_string();
        let floor = parse(&input);
        assert_eq!(floor, 0);

        let input = "(((".to_string();
        let floor = parse(&input);
        assert_eq!(floor, 3);

        let input = ")())())".to_string();
        let floor = parse(&input);
        assert_eq!(floor, -3);
    }
}
