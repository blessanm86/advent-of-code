use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
    action: String,
}

fn parse_input(input: &Vec<&str>) -> Vec<Instruction> {
    let regex = Regex::new(
        r"(?P<operation>\D+)\s(?P<startX>\d+),(?P<startY>\d+)\D+(?P<endX>\d+),(?P<endY>\d+)",
    )
    .unwrap();

    input
        .iter()
        .map(|&ins| {
            let capture = regex.captures(&ins).unwrap();

            Instruction {
                start_row: capture
                    .name("startX")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                start_col: capture
                    .name("endX")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                end_row: capture
                    .name("startY")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                end_col: capture
                    .name("endY")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                action: capture
                    .name("operation")
                    .map_or("", |m| m.as_str())
                    .trim()
                    .to_string(),
            }
        })
        .collect()
}

struct Matrix<T> {
    matrix: HashMap<String, T>,
}

impl<T: Copy + Clone> Matrix<T> {
    fn new(rows: u32, columns: u32, initial_value: T) -> Matrix<T> {
        let mut matrix = HashMap::new();

        for row_count in 0..rows {
            for column_count in 0..columns {
                matrix.insert(format!("{},{}", row_count, column_count), initial_value);
            }
        }

        Matrix { matrix }
    }

    fn get(&self, row: u32, column: u32) -> &T {
        let key = format!("{},{}", row, column);
        self.matrix.get(&key).unwrap()
    }

    fn set(&mut self, row: u32, column: u32, value: fn(x: T) -> T) {
        let key = format!("{},{}", row, column);
        let existing_value = self.get(row, column);
        self.matrix.insert(key, value(existing_value));
    }

    fn set_range(
        &mut self,
        rowStart: u32,
        columnStart: u32,
        rowEnd: u32,
        columnEnd: u32,
        value: fn(x: T) -> T,
    ) {
        for i in rowStart..rowEnd + 1 {
            for j in columnStart..columnEnd + 1 {
                self.set(i, j, value);
            }
        }
    }

    fn get_values(&self) -> Vec<T> {
        self.matrix.into_values().collect()
    }
}

fn solve(input: &Vec<Instruction>) -> u32 {
    let mut matrix = Matrix::new(1000, 1000, false);

    input.iter().for_each(|ins| {
        if ins.action == "turn on" {
            matrix.set_range(
                ins.start_row,
                ins.start_col,
                ins.end_row,
                ins.end_col,
                |x| true,
            );
        } else if ins.action == "turn off" {
            matrix.set_range(
                ins.start_row,
                ins.start_col,
                ins.end_row,
                ins.end_col,
                |x| false,
            );
        } else {
            matrix.set_range(
                ins.start_row,
                ins.start_col,
                ins.end_row,
                ins.end_col,
                |x| !x,
            );
        }
    });

    5
}

fn main() {
    let test_input = "
    turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500
    ";

    let parsed_test_input: Vec<&str> = test_input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .collect();
    let result = solve(&parse_input(&parsed_test_input));
    println!("{}", result);
}
