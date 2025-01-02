use serde_json::Value;
use std::{env, fs};

fn solve(input: Value) -> i32 {
    match input {
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::Array(vals) => vals.into_iter().map(solve).sum(),
        Value::Object(obj) => {
            if obj.values().any(|val| val == "red") {
                0
            } else {
                obj.into_iter().map(|(_, val)| solve(val)).sum()
            }
        }
        _ => 0,
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let result = solve(serde_json::from_str(&input).unwrap());

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = "[1,2,3]".to_string();
        let result = solve(serde_json::from_str(&input).unwrap());
        assert_eq!(result, 6);

        let input = r#"[1,{"c":"red","b":2},3]"#.to_string();
        let result = solve(serde_json::from_str(&input).unwrap());
        assert_eq!(result, 4);

        let input = r#"[1,{"b":2,"c":"red"},3]"#.to_string();
        let result = solve(serde_json::from_str(&input).unwrap());
        assert_eq!(result, 4);

        let input = r#"{"d":"red","e":[1,2,3,4],"f":5}"#.to_string();
        let result = solve(serde_json::from_str(&input).unwrap());
        assert_eq!(result, 0);

        let input = r#"[1,"red",5]"#.to_string();
        let result = solve(serde_json::from_str(&input).unwrap());
        assert_eq!(result, 6);
    }
}
