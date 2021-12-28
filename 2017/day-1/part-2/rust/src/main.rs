use std::collections::HashSet;
use std::fs;

fn solve(input: &str) -> u32 {
    let mut count = 0;
    let digits: Vec<u32> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let digits = &digits;

    let temp = &format!("{}{}", &input, &input);
    let temp_digits: Vec<u32> = temp
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();
    let limit = digits.len() / 2;

    for i in 0..digits.len() {
        let is_equal = temp_digits[i..i + limit + 1].eq(&temp_digits[i + 1..i + limit + 2]);
        println!("{}", is_equal);
        count += if diff.len() == 0 {
            // println!("{:?} {} {}", digits, i, digits[i]);
            digits[i]
        } else {
            0
        };
    }

    count
}

fn main() {
    let _input = fs::read_to_string("./src/input.txt").expect("cant read file");
    println!("{}", solve("1212"));
}
