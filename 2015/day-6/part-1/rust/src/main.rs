use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Instruction {
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
    action: String,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let input: Vec<&str> = input.trim().lines().collect();

    let regex = Regex::new(
        r"(?P<operation>\D+)\s(?P<start_row>\d+),(?P<start_column>\d+)\D+(?P<end_row>\d+),(?P<end_column>\d+)",
    )
        .unwrap();

    input
        .iter()
        .map(|&ins| {
            let capture = regex.captures(&ins).unwrap();

            Instruction {
                start_row: capture
                    .name("start_row")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                start_col: capture
                    .name("start_column")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                end_row: capture
                    .name("end_row")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                end_col: capture
                    .name("end_column")
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

    fn get(&self, row: u32, column: u32) -> T {
        let key = format!("{},{}", row, column);
        self.matrix.get(&key).unwrap().clone()
    }

    fn set(&mut self, row: u32, column: u32, value: fn(x: T) -> T) {
        let key = format!("{},{}", row, column);
        let existing_value = self.get(row, column);
        self.matrix.insert(key, value(existing_value));
    }

    fn set_range(
        &mut self,
        row_start: u32,
        column_start: u32,
        row_end: u32,
        column_end: u32,
        value: fn(x: T) -> T,
    ) {
        for i in row_start..row_end + 1 {
            for j in column_start..column_end + 1 {
                self.set(i, j, value);
            }
        }
    }

    fn _get_values(&self) -> Vec<T> {
        self.matrix.iter().map(|(_, v)| *v).collect()
    }

    fn get_count(&self, filter_fn: fn(x: T) -> bool) -> usize {
        self.matrix.iter().filter(|(_, &v)| filter_fn(v)).count()
    }
}

fn solve(input: &str) -> usize {
    let input = parse_input(&input);
    let mut matrix = Matrix::new(1000, 1000, false);

    input.iter().for_each(|ins| {
        if ins.action == "turn on" {
            matrix.set_range(
                ins.start_row,
                ins.start_col,
                ins.end_row,
                ins.end_col,
                |_| true,
            );
        } else if ins.action == "turn off" {
            matrix.set_range(
                ins.start_row,
                ins.start_col,
                ins.end_row,
                ins.end_col,
                |_| false,
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

    matrix.get_count(|x| x)
}

fn main() {
    let test_input = "
    turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500
    ";

    let result = solve(&test_input);
    println!("{}", result); //998996

    let input = fs::read_to_string("./src/input.txt").expect("error reading file");
    let result = solve(&input);
    println!("{}", result); //377891
}
