use md5;
use regex::Regex;

fn solve(secret: &str) -> u32 {
    let mut counter = 0;
    let rx = Regex::new(r"^0+").unwrap();

    loop {
        let secret = format!("{}{}", secret, counter);
        let hash = md5::compute(&secret);
        let hash = format!("{:?}", hash);
        let result = rx.captures(&hash);

        match result {
            Some(num) => {
                if num.get(0).unwrap().end() >= 6 {
                    return counter;
                }
            }
            None => {}
        }

        counter += 1;
    }
}

fn main() {
    assert_eq!(solve("iwrupvqb"), 9958218);
    println!("{}", solve("iwrupvqb"));
}
