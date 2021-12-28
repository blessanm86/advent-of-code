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
                if num.get(0).unwrap().end() >= 5 {
                    return counter;
                }
            }
            None => {}
        }

        counter += 1;
    }
}

fn main() {
    assert_eq!(solve("abcdef"), 609043);
    assert_eq!(solve("pqrstuv"), 1048970);
    assert_eq!(solve("iwrupvqb"), 346386);
    println!("{}", solve("iwrupvqb"));
}
