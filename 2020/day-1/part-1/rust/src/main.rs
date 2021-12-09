use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        parsed_input.push(num.trim().parse::<i32>().unwrap());
    }

    parsed_input
}

fn get_pairs(expenses: &Vec<i32>, total: i32) -> (i32, i32) {
    let mut outer_counter = 0;
    let mut inner_counter = outer_counter + 1;

    'outer: loop {
        loop {
            if inner_counter == expenses.len() {
                break;
            }

            if expenses[outer_counter] + expenses[inner_counter] == total {
                break 'outer (expenses[outer_counter], expenses[inner_counter]);
            }

            inner_counter += 1;
        }

        outer_counter += 1;
        inner_counter = outer_counter + 1;
    }
}

fn main() {
    let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");

    let parsed_test_input = parse_input(&String::from(test_input));
    assert_eq!(get_pairs(&parsed_test_input, 2020), (1721, 299));

    let parsed_input = parse_input(&String::from(input));
    assert_eq!(get_pairs(&parsed_input, 2020), (618, 1402));
}
