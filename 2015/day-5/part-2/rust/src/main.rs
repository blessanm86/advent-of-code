use fancy_regex::Regex;
use std::fs;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn is_nice(str: &str) -> bool {
    let match_pair = Regex::new(r"(\w\w).*\1").unwrap().is_match(str).unwrap();
    if !match_pair {
        return false;
    }

    let is_repeating_chars = Regex::new(r"(\w)\w\1").unwrap().is_match(str).unwrap();
    if !is_repeating_chars {
        return false;
    }

    true
}

fn main() {
    assert_eq!(is_nice("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_nice("xxyxx"), true);
    assert_eq!(is_nice("uurcxstgmygtbstg"), false);
    assert_eq!(is_nice("ieodomkazucvgmuy"), false);

    let input = fs::read_to_string("./src/input.txt").expect("error reading file");
    let count = parse_input(&input)
        .iter()
        .filter(|str| is_nice(str))
        .count();
    println!("Count - {}", count); //55
}
