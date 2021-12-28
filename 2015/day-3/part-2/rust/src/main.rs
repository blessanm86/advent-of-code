use std::collections::HashMap;
use std::fs;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn solve(ins: &str) -> usize {
    let mut houses: HashMap<Position, u16> = HashMap::new();
    let mut start_santa_x = 0;
    let mut start_santa_y = 0;
    let mut start_robo_x = 0;
    let mut start_robo_y = 0;
    let mut lucky_house_count = 0;

    houses.insert(Position { x: 0, y: 0 }, 1);

    ins.chars().enumerate().for_each(|(index, char)| {
        let pos;

        if index % 2 == 0 {
            match char {
                '>' => {
                    start_robo_x += 1;
                }
                '<' => {
                    start_robo_x -= 1;
                }
                '^' => {
                    start_robo_y += 1;
                }
                'v' => {
                    start_robo_y -= 1;
                }
                _ => {}
            };

            pos = Position {
                x: start_robo_x,
                y: start_robo_y,
            };
        } else {
            match char {
                '>' => {
                    start_santa_x += 1;
                }
                '<' => {
                    start_santa_x -= 1;
                }
                '^' => {
                    start_santa_y += 1;
                }
                'v' => {
                    start_santa_y -= 1;
                }
                _ => {}
            };

            pos = Position {
                x: start_santa_x,
                y: start_santa_y,
            };
        }

        let count = houses.get(&pos);
        let updated_count = match count {
            Some(count) => {
                lucky_house_count += 1;
                count + 1
            }
            None => 1,
        };

        houses.insert(pos, updated_count);
    });

    houses.iter().count()
}

fn main() {
    assert_eq!(solve("^v"), 3);
    assert_eq!(solve("^>v<"), 3);
    assert_eq!(solve("^v^v^v^v^v"), 11);

    let input = fs::read_to_string("./src/input.txt").expect("error reading file");
    println!("{}", solve(&input)); //2631
}
