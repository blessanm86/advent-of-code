use std::fs;

fn parse_input(input: &String) -> Vec<Vec<&str>> {
    let mut parsed_input: Vec<Vec<&str>> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        let dimensions: Vec<&str> = num.trim().split("x").collect();
        parsed_input.push(dimensions);
    }

    parsed_input
}

fn get_paper_area(length: u32, width: u32, height: u32) -> u32 {
    let top_area = length * width;
    let width_area = width * height;
    let length_area = length * height;

    let areas = [top_area, width_area, length_area];
    let min_area = areas.iter().min().unwrap();
    return 2 * top_area + 2 * width_area + 2 * length_area + min_area;
}

fn main() {
    assert_eq!(get_paper_area(2, 3, 4), 58);
    assert_eq!(get_paper_area(1, 1, 10), 43);

    let puzzle_input =
        fs::read_to_string("./src/puzzle-input.txt").expect("Error reading the file");
    parse_input(&puzzle_input);
}
