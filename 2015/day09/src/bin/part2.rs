use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> (HashSet<&str>, HashMap<(&str, &str), u32>) {
    let mut cities = HashSet::new();
    let mut distances = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let parts = line.split("=").collect::<Vec<_>>();
        let distance = parts[1].trim().parse::<u32>().unwrap();
        let nodes = parts[0].split("to").collect::<Vec<_>>();
        let node0 = nodes[0].trim();
        let node1 = nodes[1].trim();

        cities.insert(node0);
        cities.insert(node1);
        distances.insert((node0, node1), distance);
        distances.insert((node1, node0), distance);
    }

    (cities, distances)
}

fn solve(cities: HashSet<&str>, distances: HashMap<(&str, &str), u32>) -> u32 {
    let mut longest_distance = 0;
    let n_cities = cities.len();
    let routes = cities
        .into_iter()
        .permutations(n_cities)
        .collect::<Vec<_>>();

    for route in routes {
        let mut distance = 0;

        for stops in route.windows(2) {
            if let Some(d) = distances.get(&(stops[0], stops[1])) {
                distance += *d;
            }
        }

        if distance > longest_distance {
            longest_distance = distance;
        }
    }

    longest_distance
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"input/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let (cities, distances) = parse(&input);
    let shortest_route = solve(cities, distances);

    println!("Shortest Route: {shortest_route}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141"
            .to_string();
        let (cities, distances) = parse(&input);

        assert_eq!(cities.len(), 3);
        assert_eq!(cities, HashSet::from(["London", "Dublin", "Belfast"]));
        assert_eq!(distances.len(), 6);
        assert_eq!(
            distances,
            HashMap::from([
                (("London", "Dublin"), 464),
                (("Dublin", "London"), 464),
                (("London", "Belfast"), 518),
                (("Belfast", "London"), 518),
                (("Dublin", "Belfast"), 141),
                (("Belfast", "Dublin"), 141),
            ])
        );
    }

    #[test]
    fn it_solves() {
        let input = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141"
            .to_string();
        let (cities, distances) = parse(&input);
        let longest_route = solve(cities, distances);

        assert_eq!(longest_route, 982);
    }
}
