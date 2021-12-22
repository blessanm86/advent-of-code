use std::fs;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(frequencies: &[i32]) -> i32 {
    frequencies.iter().fold(0, |acc, freq| acc + freq)
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Couldn't read the file");
    assert_eq!(solve(&parse_input(&input)), 516)
}
