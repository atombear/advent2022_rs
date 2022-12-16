use std::path::PathBuf;

use std::collections::HashSet;


const WINDOW: i64 = 4000000;


type Point = (i64, i64);

use crate::utils::read_lines;


fn parse_input_str(str: &String) -> (Point, Point) {
    let sensor_beacon_strs: Vec<&str> = str.split(": ").collect();
    let sensor_str: String = sensor_beacon_strs[0].split("Sensor at ").nth(1).unwrap().to_string();
    let beacon_str: String = sensor_beacon_strs[1].split("closest beacon is at ").nth(1).unwrap().to_string();

    let mut cache: Vec<i64> = vec![];
    for eq in sensor_str.split(", ").chain(beacon_str.split(", ")) {
        let val: i64 = eq.split("=").nth(1).unwrap().parse::<i64>().unwrap();
        cache.push(val);
    }
    assert_eq!(cache.len(), 4);
    return ((cache[0], cache[1]), (cache[2], cache[3]))
}


fn distance(p0: Point, p1: Point) -> i64 { return (p0.0-p1.0).abs() + (p0.1-p1.1).abs() }


fn in_bounds(p: Point) -> bool { return 0 <= p.0 && p.0 <= WINDOW && 0 <= p.1 && p.1 <= WINDOW }


fn get_circle(center: Point, radius: i64) -> Vec<Point> {
    let xc: i64;
    let yc: i64;
    (xc, yc) = center;

    let mut ret: Vec<Point> = vec![];

    let mut x = xc;
    let mut y = yc + radius;

    while (x, y) != (xc+radius, yc) {
        if in_bounds((x, y)) { ret.push((x, y)); }
        x += 1;
        y -= 1;
    }
    while (x, y) != (xc, yc - radius) {
        if in_bounds((x, y)) { ret.push((x, y)); }
        x -= 1;
        y -= 1;
    }
    while (x, y) != (xc - radius, yc) {
        if in_bounds((x, y)) { ret.push((x, y)); }
        x -= 1;
        y += 1;
    }
    while (x, y) != (xc, yc + radius) {
        if in_bounds((x, y)) { ret.push((x, y)); }
        x += 1;
        y += 1;
    }
    return ret
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input15.txt".to_string()
    ].iter().collect();

    let mut sensor: Point;
    let mut beacon: Point;

    let select_y: i64 = WINDOW / 2;
    let mut known_points: HashSet<Point> = HashSet::new();
    let mut all_sensors: HashSet<Point> = HashSet::new();
    let mut all_beacons: HashSet<Point> = HashSet::new();
    let mut all_sensor_beacons: HashSet<(Point, Point, i64)> = HashSet::new();

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(sensor_info_str) = line {
                (sensor, beacon) = parse_input_str(&sensor_info_str);
                let dist: i64 = distance(sensor, beacon);

                all_sensors.insert(sensor);
                all_beacons.insert(beacon);
                all_sensor_beacons.insert((sensor, beacon, dist));

                let mut d: i64 = 0;
                while (sensor.1 - select_y).abs() + d <= dist {
                    known_points.insert((sensor.0 + d, select_y));
                    known_points.insert((sensor.0 - d, select_y));
                    d += 1;
                }
            }
        }
    }

    let num_points: u64 = known_points.iter().filter(|x| !all_beacons.contains(x) && !all_sensors.contains(x)).map(|_| 1).sum();

    // the beacon must be at the border of some sensor's covered region.
    let mut missing_beacon: Point = (-1, -1);
    let mut found: bool = false;
    for (sensor, _, dist) in all_sensor_beacons.iter() {
        for perim_point in get_circle(*sensor, dist + 1).iter() {
            match all_sensor_beacons.iter().filter(|(other_sensor, _, other_dist)| distance(*other_sensor, *perim_point) <= *other_dist).next() {
                None => { missing_beacon = *perim_point; found = true; },
                _ => {},
            }
        }
        if found { break }
    }

    return (14, num_points, (missing_beacon.0 * 4000000 + missing_beacon.1) as u64)
}
