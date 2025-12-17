use std::fs;
use itertools::{Itertools, iproduct};

use raqote::*;
#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> i64 {
        (1 + (self.x - other.x).abs()) * (1 + (self.y - other.y).abs())
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
        let i2 = (i1+1) % points.len();
        let tp2 = &points[i2];
        // test points are between in the x direction so a vertical segment
        if is_ordered(p1.x, tp1.x, p2.x) && is_ordered(p1.x, tp2.x, p2.x) 
            && tp1.x == tp2.x
            && (is_weak_ordered_4(tp1.y, p1.y, p2.y, tp2.y)
              || is_weak_ordered_4(tp2.y, p1.y, p2.y, tp1.y)
            ){
                return false
        }
        // test points are between in the y direction so a horizontal segment
        if is_ordered(p1.y, tp1.y, p2.y) && is_ordered(p1.y, tp2.y, p2.y) 
            && tp1.y == tp2.y
            && (
                is_weak_ordered_4(tp1.x, p1.x, p2.x, tp2.x)
                || is_weak_ordered_4(tp2.x, p1.x, p2.x, tp1.x)
            ){
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

fn intersect(p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> bool{
    // does the line (p0, p1) intersect (p2, p3)
    return false
}

// fn _main(){
//     // let filename = "example.txt".to_string();
//     let filename = "input.txt".to_string();
//     let points = read_input(filename); 

//     let mut best_p0: Option<Point> = None;
//     let mut best_p1: Option<Point> = None;
//     let mut best_area = 0;

//     let point_pairs= iproduct!(points.clone(), points.clone())
//       .sorted_by(|(a, b), (c,d)| a.area(b).cmp(&c.area(d)));
//     for (i, (p0, p1)) in point_pairs.enumerate(){
//         println!("{}", i);
//         let area = p0.area(&p1);
//         let mut intersects = false;
//         for (p2, p3) in iproduct!(points.clone(), points.clone()){
//             if intersect(&p0, &p1, &p2, &p3){
//                 intersects = true;
//             }
//         }
//         if !intersects && area > best_area{
//             best_area = area;
//             best_p0 = Some(Point { x: p0.x, y: p0.y });
//             best_p1 = Some(Point { x: p1.x, y: p1.y });
//             break;
//         }
//     }


//     println!("{:?} {:?} {}", best_p0, best_p1, best_area)
// }

fn main() {
    // let filename = "example.txt".to_string();
    let filename = "input.txt".to_string();
    let points = read_input(filename); 
    println!("Answer 1 {:?}", (iproduct!(points.clone(), points.clone()).map(|(p1, p2)| p1.area(&p2))).max());
    let good_rects = iproduct!(points.clone(), points.clone())
      .filter(|(p1, p2)| !point_inside(p1, p2, &points))
      .filter(|(p1,p2)| green_tiles(p1,p2, &points))
    //   .filter(|(p1,p2)| is_inner_rect(p1,p2, &points))
      ;
    let green_max= good_rects.clone()
      .map(|(p1, p2)| p1.area(&p2))
      .max()
      .unwrap();
    println!("Green max {:?}", green_max);
    let best_points = good_rects.filter(|(p1, p2)| p1.area(&p2) == green_max);
    println!("Best points: {:?}", best_points.collect::<Vec<(Point, Point)>>())
}

fn _main(){
    // For visualisation
    let filename = "input.txt".to_string();
    // let filename = "example.txt".to_string();

    let RES_X: i32 = 1000;
    let RES_Y: i32 = 1000;
    let points = read_input(filename.clone()); 
    let max_point = Point{
        x: points.iter().map(|p| p.x).max().unwrap(),
        y: points.iter().map(|p| p.y).max().unwrap(),
    };
    // let min_point = Point{
    //     x: points.iter().map(|p| p.x).min().unwrap(),
    //     y: points.iter().map(|p| p.y).min().unwrap(),
    // };

    let x_scale = (1.1*(max_point.x - 0) as f64)/(RES_X as f64);
    let y_scale = (1.1*(max_point.y - 0) as f64)/(RES_Y as f64);

    let mut dt = DrawTarget::new(RES_X, RES_Y);

    let mut pb = PathBuilder::new();
    pb.move_to(((points[0].x as f64)/x_scale) as f32, ((points[0].y as f64)/y_scale) as f32);
    for point in points.iter(){
        pb.line_to(((point.x as f64)/x_scale) as f32, ((point.y as f64)/y_scale) as f32);
    }
    pb.close();
    let path = pb.finish();

    let source= Source::Solid(SolidSource {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
        a: 0xFF,
    });
    dt.fill(&path, &source, &DrawOptions::new());

    // draw circles at points
    let source= Source::Solid(SolidSource {
        r: 0xFF,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    });

    for point in points.iter(){
        let radius = 3.0;
        let mut pb = PathBuilder::new();
        pb.arc(((point.x as f64)/x_scale) as f32, ((point.y as f64)/y_scale) as f32, radius, 0., 2. * std::f32::consts::PI);
        pb.close();
        let path = pb.finish();
        dt.fill(&path, &source, &DrawOptions::new());
    }


    let new_filename = format!(
        "{}_blank_img.png",
        filename.clone().strip_suffix(".txt").unwrap_or(&filename)
    );
    let _ = dt.write_png(new_filename);

    // Draw in winning rect:

    // let point_1 = Point { x: 83903, y: 85123 };
    // let point_2 = Point { x: 15099, y: 16084 };

    // let point_1 = Point { x: 16155, y: 83805 };
    // let point_2 = Point { x: 84138, y: 15900 };

    // let point_1 = Point { x: 94870, y: 48753 };
    // let point_2 = Point { x: 5606, y: 67703 };

    let point_1 = Point { x: 5606, y: 67703 };
    let point_2 = Point { x: 94870, y: 50025 };

    // let point_1 = Point { x: 9, y: 5 };
    // let point_2 = Point { x: 2, y: 3 };

    let rect_points: Vec<Point> = vec![
        Point{
            x: std::cmp::max(point_1.x, point_2.x),
            y: std::cmp::max(point_1.y, point_2.y),
        },
        Point{
            x: std::cmp::max(point_1.x, point_2.x),
            y: std::cmp::min(point_1.y, point_2.y),
        },
        Point{
            x: std::cmp::min(point_1.x, point_2.x),
            y: std::cmp::min(point_1.y, point_2.y),
        },
        Point{
            x: std::cmp::min(point_1.x, point_2.x),
            y: std::cmp::max(point_1.y, point_2.y),
        },
    ];

    let mut pb = PathBuilder::new();
    pb.move_to(((rect_points[0].x as f64)/x_scale) as f32, ((rect_points[0].y as f64)/y_scale) as f32);
    for point in rect_points.iter(){
        pb.line_to(((point.x as f64)/x_scale) as f32, ((point.y as f64)/y_scale) as f32);
    }
    pb.close();
    let path = pb.finish();

    let source= Source::Solid(SolidSource {
        r: 0x00,
        g: 0x00,
        b: 0xFF,
        a: 0xFF,
    });
    dt.fill(&path, &source, &DrawOptions::new());

    let new_filename = format!(
        "{}_img.png",
        filename.clone().strip_suffix(".txt").unwrap_or(&filename)
    );
    let _ = dt.write_png(new_filename);
}