use std::cmp::{max, min};
use std::path::PathBuf;
use std::collections::HashSet;

use crate::utils::read_lines;

type Point = (i64, i64, i64);

fn parse_loc(loc_str: &String) -> Point {
    let loc_vec: Vec<i64> = loc_str.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    return (loc_vec[0], loc_vec[1], loc_vec[2])
}

fn update_surfaces(all_surfaces: &mut HashSet<Point>, point: &Point) {
    let x: i64;
    let y: i64;
    let z: i64;
    (x, y, z) = *point;

    let mut pvec: Vec<i64>;
    let mut new_point: Point;

    for delta in [-1, 1] {
        for idx in 0..3 {
            pvec = vec![10 * x, 10 * y, 10 * z];
            pvec[idx] += 5 * delta;
            new_point = (pvec[0], pvec[1], pvec[2]);

            if all_surfaces.contains(&new_point) {
                all_surfaces.remove(&new_point);
            } else {
                all_surfaces.insert(new_point);
            }
        }
    }
}


fn update_cubes(all_cubes: &mut HashSet<Point>, point: &Point) {
    let x: i64;
    let y: i64;
    let z: i64;
    (x, y, z) = *point;
    all_cubes.insert((10*x, 10*y, 10*z));
}


fn find_free_space(all_cubes: &HashSet<Point>, point: &Point) -> Point {
    let x: i64;
    let y: i64;
    let z: i64;
    (x, y, z) = *point;

    return if x % 10 != 0 {
        if all_cubes.contains(&(x + 5, y, z)) { (x - 5, y, z) } else { (x + 5, y, z) }
    } else if y % 10 != 0 {
        if all_cubes.contains(&(x, y + 5, z)) { (x, y - 5, z) } else { (x, y + 5, z) }
    } else {
        if all_cubes.contains(&(x, y, z + 5)) { (x, y, z - 5) } else { (x, y, z + 5) }
    }
}


fn adj_points(point: Point) -> HashSet<Point> {
    let x: i64;
    let y: i64;
    let z: i64;
    (x, y, z) = point;
    return [
        (x+10,y,z), (x-10,y,z),
        (x,y+10,z), (x,y-10,z),
        (x,y,z+10), (x,y,z-10)].iter().map(|x| *x).collect()
}


fn is_point_free(point: &Point,
                 minx: i64,
                 miny: i64,
                 minz: i64,
                 maxx: i64,
                 maxy: i64,
                 maxz: i64) -> bool {
    let x: i64;
    let y: i64;
    let z: i64;
    (x, y, z) = *point;
    return x < minx || x > maxx || y < miny || y > maxy || z < minz || z > maxz
}


fn is_free(all_cubes: &HashSet<Point>,
           free_points: &mut HashSet<Point>,
           trapped_points: &mut HashSet<Point>,
           minx: i64,
           miny: i64,
           minz: i64,
           maxx: i64,
           maxy: i64,
           maxz: i64,
           point: &Point) -> bool {

    let free_cube: Point = find_free_space(all_cubes, point);
    let mut points: Vec<Point> = vec![free_cube];
    let mut search_points: HashSet<Point> = HashSet::new();

    while let Some(p) = points.pop() {

        if trapped_points.contains(&p) { return false }

        if free_points.contains(&p) || is_point_free(&p, minx, miny, minz, maxx, maxy, maxz) {
            free_points.extend(search_points);
            return true
        }

        search_points.insert(p);

        points.extend(
            adj_points(p)
            .iter().filter(
            |x|
                !all_cubes.contains(x) &&
                !search_points.contains(x))
        )
    }

    trapped_points.extend(search_points);

    return false
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input18.txt".to_string()
    ].iter().collect();

    let mut all_surfaces: HashSet<Point> = HashSet::new();
    let mut all_cubes: HashSet<Point> = HashSet::new();
    let mut point: Point;

    let mut minx: i64 = 1000000;
    let mut maxx: i64 = 0;
    let mut miny: i64 = 1000000;
    let mut maxy: i64 = 0;
    let mut minz: i64 = 1000000;
    let mut maxz: i64 = 0;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(loc_str) = line {
                point = parse_loc(&loc_str);
                update_surfaces(&mut all_surfaces, &point);
                update_cubes(&mut all_cubes, &point);

                minx = min(minx, 10*point.0);
                miny = min(miny, 10*point.1);
                minz = min(minz, 10*point.2);

                maxx = max(maxx, 10*point.0);
                maxy = max(maxy, 10*point.1);
                maxz = max(maxz, 10*point.2);
            }
        }
    }

    let mut num_free_edges: u64 = 0;
    let mut free_points: HashSet<Point> = HashSet::new();
    let mut trapped_points: HashSet<Point> = HashSet::new();
    for p in &all_surfaces {
        if is_free(&all_cubes,
                   &mut free_points,
                   &mut trapped_points,
                   minx,
                   miny,
                   minz,
                   maxx,
                   maxy,
                   maxz,
                   p) {
            num_free_edges += 1;
        }
    }

    return (17, all_surfaces.len() as u64, num_free_edges)
}
