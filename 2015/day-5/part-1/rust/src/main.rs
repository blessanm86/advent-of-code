use fancy_regex::Regex;
use std::fs;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn is_nice(str: &str) -> bool {
    let vowel_count = Regex::new(r"[aeiou]").unwrap().find_iter(str).count();
    if vowel_count < 3 {
        return false;
    }

    let is_repeating_chars = Regex::new(r"(\w)\1").unwrap().is_match(str).unwrap();
    if !is_repeating_chars {
        return false;
    }

    let is_match_pattern = Regex::new(r"ab|cd|pq|xy").unwrap().is_match(str).unwrap();
    if is_match_pattern {
        return false;
    }

    true
}

fn main() {
    assert_eq!(is_nice("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice("aaa"), true);
    assert_eq!(is_nice("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice("dvszwmarrgswjxmb"), false);

    let input = fs::read_to_string("./src/input.txt").expect("error reading file");
    let count = parse_input(&input)
        .iter()
        .filter(|str| is_nice(str))
        .count();
    println!("Count - {}", count); //255
}
