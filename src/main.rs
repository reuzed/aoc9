use std::fs;
use itertools::{Itertools, iproduct};

#[derive(Clone)]
#[derive(Debug)]
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

fn is_ordered(a: i64, b: i64, c:i64) -> bool {
    return (a-b) * (b-c) > 0;
}

fn is_weak_ordered_4(a: i64, b: i64, c:i64, d:i64) -> bool {
    if a >= b && b >= c && c >= d {
        return true;
    }
    if a <= b && b <= c && c <= d {
        return true;
    }
    return false;
}

fn point_inside(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    for point in points.iter(){
        if is_ordered(p1.x, point.x, p2.x) && is_ordered(p1.y, point.y, p2.y){
            // println!("Inside: {:?} {:?} {:?}", p1, point, p2);
            return true;
        }
    }
    return false;
}

fn green_tiles(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    // How do we find out whether a rectangle is all contained in the green tiles
    // Test if the interior of any segment intersects with the inside of the rectangle.
    // We can guarantee that no point is inside the rectangle
    // Ignore annoying case where 2 segments are adjacent
    for (i1, tp1) in points.iter().enumerate(){
        let i2 = (i1+1)% points.len();
        let tp2 = &points[i2];
        // 
        if is_ordered(p1.x, tp1.x, p2.x) && is_ordered(p1.x, tp2.x, p2.x) 
            && is_weak_ordered_4(tp1.y, p1.y, p2.y, tp2.y){
                return false
        }
        if is_ordered(p1.y, tp1.y, p2.y) && is_ordered(p1.y, tp2.y, p2.y) 
            && is_weak_ordered_4(tp1.x, p1.x, p2.x, tp2.x){
                return false
        }
    }
    return true;
}

fn is_inside(p: &Point, points: &Vec<Point>) -> bool {
    // Determine whether a point lies on the inside of the tile region or outside.
    // This is a global condition. 
    // We can use the test of moving past boundary edges, go upwards and counnt left closed right open segments
    let mut boundary_count = 0;
    for (i1, p1) in points.iter().enumerate(){
        let i2 = (i1+1) % points.len();
        let p2 = &points[i2];
        if p1.y == p2.y && p1.y > p.y {
            if p1.x <= p.x && p.x < p2.x {
                boundary_count += 1;
            }
            if p2.x <= p.x && p.x < p1.x {
                boundary_count += 1;
            }
        }
    }
    return (boundary_count % 2)==1;
}

fn is_inner_rect(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    let test_x = match p1.x < p2.x {
        true => p1.x + 1,
        false => p2.x + 1,
    };
    let test_y = match p1.y < p2.y {
        true => p1.y + 1,
        false => p2.y + 1,
    };
    let test_point = Point { x: test_x, y: test_y };
    let is_ir= is_inside(&test_point, points);
    // println!("Test point for {:?} {:?} {:?} - {}", p1,p2,test_point, is_ir);
    return is_ir;
}

fn main() {
    // let filename = "example.txt".to_string();
    let filename = "input.txt".to_string();
    let points = read_input(filename); 
    println!("Answer 1 {:?}", (iproduct!(points.clone(), points.clone()).map(|(p1, p2)| p1.area(&p2))).max());
    let good_rects = iproduct!(points.clone(), points.clone())
      .filter(|(p1, p2)| !point_inside(p1, p2, &points))
      .filter(|(p1,p2)| green_tiles(p1,p2, &points))
      .filter(|(p1,p2)| is_inner_rect(p1,p2, &points));
    let green_max= good_rects.clone()
      .map(|(p1, p2)| p1.area(&p2))
      .max()
      .unwrap();
    println!("Green max {:?}", green_max);
    let best_points = good_rects.filter(|(p1, p2)| p1.area(&p2) == green_max);
    println!("Best points: {:?}", best_points.collect::<Vec<(Point, Point)>>())
}