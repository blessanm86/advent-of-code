use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve(frequencies: &[i32]) -> i32 {
    let mut temp_results: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;
    let mut index = 0;
    let end = frequencies.len() - 1;

    let result = loop {
        let new_freq = sum + frequencies[index];
        let result = temp_results.get(&new_freq);

        match result {
            Some(..) => {
                break new_freq;
            }
            None => {
                temp_results.insert(new_freq, 1);
            }
        }

        sum += frequencies[index];

        index = if index == end { 0 } else { index + 1 }
    };

    result
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Couldn't read the file");
    assert_eq!(solve(&parse_input(&input)), 71892)
}
