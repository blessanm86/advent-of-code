use std::fs;

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|num| num.parse().unwrap()).collect()
}

fn solve(masses: &[u32]) -> u32 {
    masses
        .iter()
        .map(|mass| (mass / 3) - 2)
        .fold(0, |acc, fuel| acc + fuel)
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");
    let parsed_input = parse_input(&input);
    assert_eq!(solve(&parsed_input), 3358992);
}
