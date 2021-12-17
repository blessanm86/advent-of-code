use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        parsed_input.push(num.trim().parse::<i32>().unwrap());
    }

    parsed_input
}

fn solve(expenses: &Vec<i32>, count: i32, total: i32, sum: i32) -> Vec<i32> {
    //JS code. Need to learn more about closures and iterators in rust.

    if count == 1 {
        let r = expenses.iter().find(|&exp| *exp + sum == total).unwrap();
        let mut vec: Vec<i32> = Vec::new();
        vec.push(r);
        println!("{}", r);
        return vec;
    } else {
        // for (let i = 0; i < expenses.length; i++) {
        //     const slice = expenses.slice(i + 1);
        //     const num = solve(slice, count - 1, total, expenses[i] + sum);
        //     if (num) {
        //         return [expenses[i], num].flat();
        //     }
        // }
        // let mut i = 0;
        // loop {
        //     let slice: Vec<i32> = expenses[i..];
        //     let num = solve(&slice, count - 1, total, expenses[i] + sum);
        //     println!("{:?}", num);
        // }
    }

    return Vec::new();
}

fn main() {
    // let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    // let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");
    //
    // let parsed_test_input = parse_input(&String::from(test_input));
    // assert_eq!(solve(&parsed_test_input, 2, 2020, 0), (1721, 299));
    // solve(&parsed_test_input, 2, 2020, 0);

    // let parsed_input = parse_input(&String::from(input));
    // assert_eq!(get_pairs(&parsed_input, 2020), (618, 1402));
}
