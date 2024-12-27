use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn load(input: &str) -> HashMap<&str, u16> {
    let mut state = HashMap::new();
    let instructions = input.lines().map(|line| line.trim()).collect::<Vec<_>>();
    let mut executed: HashSet<&str> = HashSet::new();

    while executed.len() != instructions.len() {
        for &inst in instructions.iter() {
            if executed.contains(inst) {
                continue;
            }

            let parts = inst.split("->").collect::<Vec<_>>();
            let left = parts[0].trim().split_whitespace().collect::<Vec<_>>();
            let out = parts[1].trim();

            match left.len() {
                1 => {
                    if left[0].parse::<u16>().is_ok() {
                        let input = left[0].parse::<u16>().unwrap();
                        state.entry(out).and_modify(|v| *v = input).or_insert(input);
                        executed.insert(inst);
                    } else {
                        if let Some(input) = state.get(&left[0]) {
                            state.insert(out, *input);
                            executed.insert(inst);
                        }
                    }
                }
                2 => {
                    if left[0] == "NOT" {
                        if let Some(input) = state.get(&left[1]) {
                            state.insert(out, !input);
                            executed.insert(inst);
                        }
                    }
                }
                3 => match left[1] {
                    "AND" => {
                        if let (Some(input0), Some(input1)) =
                            (state.get(&left[0]), state.get(&left[2]))
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        } else if let (Ok(input0), Some(input1)) =
                            (left[0].parse::<u16>(), state.get(&left[2]))
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        } else if let (Some(input0), Ok(input1)) =
                            (state.get(&left[0]), left[2].parse::<u16>())
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        }
                    }
                    "OR" => {
                        if let (Some(input0), Some(input1)) =
                            (state.get(&left[0]), state.get(&left[2]))
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        } else if let (Ok(input0), Some(input1)) =
                            (left[0].parse::<u16>(), state.get(&left[2]))
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        } else if let (Some(input0), Ok(input1)) =
                            (state.get(&left[0]), left[2].parse::<u16>())
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        }
                    }
                    "LSHIFT" => {
                        if let Some(input0) = state.get(&left[0]) {
                            let shift = left[2].parse::<u16>().unwrap();
                            state.insert(out, input0 << shift);
                            executed.insert(inst);
                        }
                    }
                    "RSHIFT" => {
                        if let Some(input0) = state.get(&left[0]) {
                            let shift = left[2].parse::<u16>().unwrap();
                            state.insert(out, input0 >> shift);
                            executed.insert(inst);
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    let wire_a = state.get("a").unwrap().clone();
    state.clear();
    executed.clear();

    // Yes I know I shouldn't copy and paste this loop again, but sometimes
    // with AoC you just go for the quick and dirty solution.
    while executed.len() != instructions.len() {
        for &inst in instructions.iter() {
            if executed.contains(inst) {
                continue;
            }

            let parts = inst.split("->").collect::<Vec<_>>();
            let left = parts[0].trim().split_whitespace().collect::<Vec<_>>();
            let out = parts[1].trim();

            match left.len() {
                1 => {
                    if left[0].parse::<u16>().is_ok() {
                        let mut input = left[0].parse::<u16>().unwrap();
                        if inst == format!("{input} -> b") {
                            input = wire_a;
                        }
                        state.entry(out).and_modify(|v| *v = input).or_insert(input);
                        executed.insert(inst);
                    } else {
                        if let Some(input) = state.get(&left[0]) {
                            state.insert(out, *input);
                            executed.insert(inst);
                        }
                    }
                }
                2 => {
                    if left[0] == "NOT" {
                        if let Some(input) = state.get(&left[1]) {
                            state.insert(out, !input);
                            executed.insert(inst);
                        }
                    }
                }
                3 => match left[1] {
                    "AND" => {
                        if let (Some(input0), Some(input1)) =
                            (state.get(&left[0]), state.get(&left[2]))
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        } else if let (Ok(input0), Some(input1)) =
                            (left[0].parse::<u16>(), state.get(&left[2]))
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        } else if let (Some(input0), Ok(input1)) =
                            (state.get(&left[0]), left[2].parse::<u16>())
                        {
                            state.insert(out, input0 & input1);
                            executed.insert(inst);
                        }
                    }
                    "OR" => {
                        if let (Some(input0), Some(input1)) =
                            (state.get(&left[0]), state.get(&left[2]))
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        } else if let (Ok(input0), Some(input1)) =
                            (left[0].parse::<u16>(), state.get(&left[2]))
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        } else if let (Some(input0), Ok(input1)) =
                            (state.get(&left[0]), left[2].parse::<u16>())
                        {
                            state.insert(out, input0 | input1);
                            executed.insert(inst);
                        }
                    }
                    "LSHIFT" => {
                        if let Some(input0) = state.get(&left[0]) {
                            let shift = left[2].parse::<u16>().unwrap();
                            state.insert(out, input0 << shift);
                            executed.insert(inst);
                        }
                    }
                    "RSHIFT" => {
                        if let Some(input0) = state.get(&left[0]) {
                            let shift = left[2].parse::<u16>().unwrap();
                            state.insert(out, input0 >> shift);
                            executed.insert(inst);
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    state
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let state = load(&input);

    println!("Wire a: {:?}", state.get(&"a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_parses() {
        let input = "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i"
            .to_string();
        let state = load(&input);

        assert_eq!(
            state,
            HashMap::from([
                ("d", 72),
                ("e", 507),
                ("f", 492),
                ("g", 114),
                ("h", 65412),
                ("i", 65079),
                ("x", 123),
                ("y", 456),
            ])
        );
    }
}
