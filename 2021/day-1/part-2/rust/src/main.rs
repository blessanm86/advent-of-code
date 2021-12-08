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
    let mut counter = 0;

    loop {
        if counter + 3 == readings.len() {
            break;
        }

        let prev_reading = readings[counter] + readings[counter + 1] + readings[counter + 2];
        let next_reading = readings[counter + 1] + readings[counter + 2] + readings[counter + 3];

        if next_reading > prev_reading {
            count += 1;
        }

        counter += 1;
    }

    count
}

fn main() {
    let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");

    assert_eq!(
        get_depth_increase_count(&parse_input(&String::from(test_input))),
        5
    );
    println!("{}", get_depth_increase_count(&parse_input(&input))); //1346
}
