use std::fs;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|num| num.parse().unwrap()).collect()
}

fn get_fuel(fuel: &i32) -> i32 {
    let needed_fuel = (fuel / 3) - 2;

    if needed_fuel > 0 {
        return needed_fuel + get_fuel(&needed_fuel);
    }

    0
}

fn solve(masses: &[i32]) -> i32 {
    masses.iter().map(get_fuel).fold(0, |acc, fuel| acc + fuel)
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");
    let parsed_input = parse_input(&input);
    assert_eq!(solve(&parsed_input), 5035632);
}
