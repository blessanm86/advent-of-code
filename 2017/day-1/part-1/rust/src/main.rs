use std::fs;

fn solve(input: &str) -> u32 {
    let mut count = 0;
    let digits: Vec<u32> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();

    for i in 0..digits.len() {
        let next_index = if i == digits.len() - 1 { 0 } else { i + 1 };

        count += if digits[i] == digits[next_index] {
            digits[i]
        } else {
            0
        };
    }

    count
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("cant read file");
    println!("{}", solve(&input));
}
