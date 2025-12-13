use std::fs;
use itertools::{Itertools, iproduct};

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> i64 {
        (1 + self.x - other.x).abs() * (1 + self.y - other.y).abs()
    }
}

fn read_input(filename: String) -> Vec<Point>{
    fs::read_to_string(filename).expect("File has content").trim()
    .split("\n").map(
        |l| l.split(",").map(|s| i64::from_str_radix(s.trim(), 10).unwrap())
    ).map(
        |l| Point{x:l.clone().nth(0).unwrap(), y:l.clone().nth(1).unwrap()}
    ).collect()
}


fn main() {
    // let filename = "example.txt".to_string();
    let filename = "input.txt".to_string();
    let points = read_input(filename); 
    println!("{:?}", (iproduct!(points.clone(), points).map(|(p1, p2)| p1.area(&p2))).max());
}