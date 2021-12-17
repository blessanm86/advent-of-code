use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    let mut parsed_input: Vec<i32> = Vec::new();

    for (_, num) in input.lines().enumerate() {
        parsed_input.push(num.trim().parse::<i32>().unwrap());
    }

    parsed_input
}

fn solve(expenses: &[i32], count: i32, total: i32, sum: i32) -> Option<&[i32]> {

    if count == 1 {
        let result = expenses.into_iter().position(|x| 2020 + sum == total );
        println!("num {}", expenses[1]);

        return match result {
            Some(x) => {
                println!("Found number: {}", x);

                Some(&expenses[x..x])
            }
            None => {
                println!("No match");
                None
            }
        }
    }


    for i in 0..expenses.len() {
        println!("{:?}", expenses[i]);
        let slice = &expenses[i + 1..];
        let num = solve(slice, count - 1, total, expenses[i] + sum);

        // return match result {
        //     Some(num) => {
        //         if num.len() > 0 {
        //             let nums = num.iter().flatten().collect();
        //             nums.push(expenses[i]);
        //             return nums;
        //         } 

        //         println!("Found number: {}", x);
        //         Some(vec![*x])
        //     }
        //     None => {
        //         println!("No match");
        //         None
        //     }
        // }
    }

    None
}

fn main() {
    let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    let _input = fs::read_to_string("./src/input.txt").expect("Error reading the file");

    let parsed_test_input = parse_input(&String::from(test_input));
    // assert_eq!(solve(&parsed_test_input, 2, 2020, 0), (1721, 299));

    let test = solve(&parsed_test_input[..], 1, 2020, 0).unwrap();
    println!("Worked {:?}", test);

    // let parsed_input = parse_input(&String::from(input));
    // assert_eq!(get_pairs(&parsed_input, 2020), (618, 1402));
}
