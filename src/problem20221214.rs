use std::cmp::{max, min};
use std::path::PathBuf;
use std::collections::HashSet;

use crate::utils::read_lines;


type Point = (u64, u64);


fn parse_wall_str_to_points(wall_str: &String) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];

    for point_str in wall_str.split(&" -> ".to_string()) {
        let point_vec: Vec<u64> = point_str.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
        ret.push((point_vec[0], point_vec[1]));
    }

    return ret
}


fn add_rock_line(all_rocks: &mut HashSet<Point>, p0: Point, p1: Point) {
    let x0: u64 = p0.0;
    let y0: u64 = p0.1;
    let x1: u64 = p1.0;
    let y1: u64 = p1.1;

    assert!(x0 == x1 || y0 == y1);

    for idx in min(x0, x1)..max(x0, x1)+1 {
        for jdx in min(y0, y1)..max(y0, y1)+1 {
            all_rocks.insert((idx, jdx));
        }
    }
}

fn down(point: Point) -> Point { (point.0, point.1+1) }
fn left(point: Point) -> Point { (point.0-1, point.1+1) }
fn right(point: Point) -> Point { (point.0+1, point.1+1) }

fn update_sand(all_rocks: &HashSet<Point>, sand: Point) -> Option<Box<dyn Fn(Point) -> Point>> {
    for mv in [down, left, right].into_iter().filter(|mv| !all_rocks.contains(&mv(sand))) {
        return Some(Box::new(mv))
    }
    return None
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input14.txt".to_string()
    ].iter().collect();

    let mut all_rocks: HashSet<Point> = HashSet::new();
    let mut points: Vec<Point>;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(wall_str) = line {
                points = parse_wall_str_to_points(&wall_str);
                for idx in 0..points.len() - 1 {
                    add_rock_line(&mut all_rocks, points[idx], points[idx+1])
                }
            }
        }
    }

    let max_y: u64 = *all_rocks.iter().map(|(_, y)| y).max().unwrap();
    let ceiling: u64 = max_y + 2;
    for x in 0..1000 { all_rocks.insert((x, ceiling)); }

    let mut num_sands: u64 = 0;

    let mut found_first_volume: bool = false;
    let mut first_volume: u64 = 0;

    let mut sand: Point = (0, 0);

    while sand != (500, 0) {
        sand = (500, 0);
        while let Some(mv) = update_sand(&all_rocks, sand) { sand = mv(sand); }

        if !found_first_volume && sand.1 >= max_y { found_first_volume=true; first_volume=num_sands; }

        all_rocks.insert(sand);
        num_sands += 1;
    }

    return (13, first_volume, num_sands)
}
