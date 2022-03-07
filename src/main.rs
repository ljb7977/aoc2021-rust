use std::fs;

fn parse_integers(s: &String) -> Vec<i32> {
    s.split("\n").filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("resources/day1.txt").expect("file read");
    let v: Vec<i32> = parse_integers(&contents);
    println!("{:?}", v);
}
