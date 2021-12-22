use std::fs;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|str| str.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn calc(expenses: &[u32], count: u32, total: u32, sum: u32) -> Option<Vec<u32>> {
    if count == 1 {
        let result = expenses.into_iter().position(|x| x + sum == total);
        return match result {
            Some(x) => Some(vec![expenses[x]]),
            None => Some(Vec::new()),
        };
    }

    for i in 0..expenses.len() {
        let result = calc(&expenses[i + 1..], count - 1, total, expenses[i] + sum);

        match result {
            Some(res) => {
                if res.len() > 0 {
                    let mut nums: Vec<u32> = vec![expenses[i]];
                    for i in res {
                        nums.push(i);
                    }

                    return Some(nums);
                }
            }
            _ => {}
        }
    }

    None
}

fn main() {
    let test_input = fs::read_to_string("./src/test_input.txt").expect("Error reading the file");
    let parsed_test_input = parse_input(&test_input);
    assert_eq!(
        calc(&parsed_test_input, 2, 2020, 0).unwrap(),
        vec![1721, 299]
    );

    let input = fs::read_to_string("./src/input.txt").expect("Error reading the file");
    let parsed_input = parse_input(&input);
    assert_eq!(
        calc(&parsed_input, 3, 2020, 0).unwrap(),
        vec![928, 547, 545]
    );
}
