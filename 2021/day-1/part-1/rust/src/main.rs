use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        parsed_input.push(num.trim().parse::<i32>().unwrap());
    }

    parsed_input
}

fn get_depth_increase_count(readings: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut counter = 1;

    loop {
        if counter == readings.len() {
            break;
        }

        if readings[counter] > readings[counter - 1] {
            count += 1;
        }

        counter += 1;
    }

    count
}

fn main() {
    let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");

    assert_eq!(get_depth_increase_count(&parse_input(&test_input)), 7);
    println!("{}", get_depth_increase_count(&parse_input(&input))); //1301
}
