use std::fs;

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    let mut parsed_input: Vec<Vec<u32>> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        let dimensions: Vec<u32> = num.trim().split("x").map(|x| x.parse().unwrap()).collect();
        parsed_input.push(dimensions);
    }

    parsed_input
}

fn get_bow_length(length: u32, width: u32, height: u32) -> u32 {
    let perimeters = [
        2 * length + 2 * width,
        2 * length + 2 * height,
        2 * width + 2 * height,
    ];

    let min_perimeter = perimeters.iter().min().unwrap();
    min_perimeter + length * width * height
}

fn solve(dimens: Vec<Vec<u32>>) -> u32 {
    dimens
        .iter()
        .map(|dim| get_bow_length(dim[0], dim[1], dim[2]))
        .fold(0, |sum, x| sum + x)
}

fn main() {
    assert_eq!(get_bow_length(2, 3, 4), 34);
    assert_eq!(get_bow_length(1, 1, 10), 14);

    let puzzle_input =
        fs::read_to_string("./src/puzzle-input.txt").expect("Error reading the file");
    let result = solve(parse_input(&puzzle_input));
    println!("{}", result);
}
