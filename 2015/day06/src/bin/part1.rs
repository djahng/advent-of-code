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

fn solve(lights: &[Light]) -> usize {
    let mut grid: HashMap<(usize, usize), bool> = HashMap::new();

    for light in lights {
        for x in light.start.0..=light.end.0 {
            for y in light.start.1..=light.end.1 {
                match light.instruction {
                    Instruction::TurnOn => {
                        grid.insert((x, y), true);
                    }
                    Instruction::TurnOff => {
                        grid.insert((x, y), false);
                    }
                    Instruction::Toggle => {
                        grid.entry((x, y)).and_modify(|v| *v = !*v).or_insert(true);
                    }
                }
            }
        }
    }

    grid.into_iter().filter(|(_, light)| *light == true).count()
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
    println!("Number of lights that are on: {n_lights}");
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

    #[test]
    fn it_lights() {
        let input = "turn on 0,0 through 999,999
        toggle 0,0 through 999,0
        turn off 499,499 through 500,500"
            .to_string();
        let lights = parse(&input);

        let n_lights = solve(&lights[0..1]);
        assert_eq!(n_lights, 1_000_000);

        let n_lights = solve(&lights[1..2]);
        assert_eq!(n_lights, 1000);

        let n_lights = solve(&lights[2..]);
        assert_eq!(n_lights, 0);
    }
}
