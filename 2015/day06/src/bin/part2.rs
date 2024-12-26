use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

#[derive(Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Light {
    instruction: Instruction,
    start: (usize, usize),
    end: (usize, usize),
}

impl Light {
    fn parse(input: &str) -> Self {
        let mut instruction = Instruction::TurnOff;
        let mut start = (0, 0);
        let mut end = (0, 0);

        let mut parts: VecDeque<_> = input.trim().split_whitespace().collect();
        while let Some(next) = parts.pop_front() {
            match next {
                "turn" => continue,
                "on" => instruction = Instruction::TurnOn,
                "off" => instruction = Instruction::TurnOff,
                "toggle" => instruction = Instruction::Toggle,
                "through" => continue,
                next if next.contains(",") => {
                    let indexes = next
                        .split(",")
                        .filter_map(|n| n.parse::<usize>().ok())
                        .collect::<Vec<_>>();

                    if parts.len() > 1 {
                        // Start index
                        start = (indexes[0], indexes[1]);
                    } else {
                        // End index
                        end = (indexes[0], indexes[1]);
                    }
                }
                _ => {}
            }
        }

        Light {
            instruction,
            start,
            end,
        }
    }
}

fn parse(input: &str) -> Vec<Light> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            Light::parse(&line)
        })
        .collect::<Vec<_>>()
}

fn solve(lights: &[Light]) -> u32 {
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();

    for light in lights {
        for x in light.start.0..=light.end.0 {
            for y in light.start.1..=light.end.1 {
                match light.instruction {
                    Instruction::TurnOn => {
                        grid.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                    }
                    Instruction::TurnOff => {
                        grid.entry((x, y))
                            .and_modify(|v| *v = v.saturating_sub(1))
                            .or_insert(0);
                    }
                    Instruction::Toggle => {
                        grid.entry((x, y)).and_modify(|v| *v += 2).or_insert(2);
                    }
                }
            }
        }
    }

    grid.values().sum()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");

    let lights = parse(&input);
    let n_lights = solve(&lights);
    println!("Total brightness: {n_lights}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_input() {
        let input = "turn on 0,0 through 999,999
        toggle 0,0 through 999,0
        turn off 499,499 through 500,500"
            .to_string();
        let lights = parse(&input);

        assert_eq!(lights.len(), 3);
        assert_eq!(
            lights[0],
            Light {
                instruction: Instruction::TurnOn,
                start: (0, 0),
                end: (999, 999),
            }
        );
        assert_eq!(
            lights[1],
            Light {
                instruction: Instruction::Toggle,
                start: (0, 0),
                end: (999, 0),
            }
        );
        assert_eq!(
            lights[2],
            Light {
                instruction: Instruction::TurnOff,
                start: (499, 499),
                end: (500, 500),
            }
        );
    }
}
