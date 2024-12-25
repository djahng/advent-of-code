use std::{env, fs};

use md5;

fn solve(secret_key: String) -> u64 {
    let secret_key = secret_key.trim();
    let mut num = 0;

    loop {
        let base = format!("{}{}", secret_key, num);
        let digest = md5::compute(base.as_bytes());
        let digest = format!("{:x}", digest);

        if digest.starts_with("000000") {
            break;
        }
        num += 1;
    }
    num
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let result = solve(input);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_solves() {
        let input = "abcdef".to_string();
        let result = solve(input);

        assert_eq!(result, 609043);
    }
}
