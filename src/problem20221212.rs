use std::cmp::min;
use std::path::PathBuf;

use std::collections::HashSet;

use crate::utils::read_lines;


type Point = (usize, usize);


fn index(word: String, search: char) -> Option<usize> {
    for (idx, _) in word.chars().enumerate().filter(|(_, c)| c == &search) {
        return Some(idx)
    }
    return None
}


fn char_to_height(val: char) -> usize {
    return if val == 'S' { 0 } else if val == 'E' { 25 } else { index("abcdefghijklmnopqrstuvwxyz".to_string(), val).unwrap() }
}


fn get_adj_points(pt: Point, map: &Vec<Vec<char>>) -> Vec<Point> {
    let r: usize;
    let c: usize;
    (r, c) = pt;
    let rows = map.len();
    let cols = map[0].len();

    let mut all_points: Vec<Point> = vec![];
    if r > 0 { all_points.push((r-1, c)) };
    if c > 0 { all_points.push((r, c-1)) };
    if r + 1 < rows { all_points.push((r+1, c)) };
    if c + 1 < cols { all_points.push((r, c+1)) };

    let mut ret : Vec<Point> = vec![];

    let val = char_to_height(map[r][c]);
    for (new_r, new_c) in all_points.iter().filter(|(new_r, new_c)| val + 1 >= char_to_height(map[*new_r][*new_c])) {
        ret.push((*new_r, *new_c));
    }

    return ret
}


fn find_dist(start: Point, map: &Vec<Vec<char>>) -> u64 {

    let mut visited: HashSet<Point> = HashSet::new();
    let mut next_points: Vec<Point> = vec![];

    visited.insert(start );
    next_points.push(start);

    let mut points_cache: Vec<Point> = vec![];
    let mut solved: bool = false;
    let mut steps: u64 = 0;

    while !solved {
        steps += 1;

        if next_points.len() == 0 { return 1000 }

        while let Some(point) = next_points.pop() {

            for next_point in get_adj_points(point, &map) {
                if !visited.contains(&next_point) {
                    let next_r: usize;
                    let next_c: usize;
                    (next_r, next_c) = next_point;

                    if map[next_r][next_c] == 'E' { solved=true; break }

                    visited.insert(next_point);
                    points_cache.push(next_point);
                }
            }
            if solved { break }
        }

        while let Some(point) = points_cache.pop() {
            next_points.push(point);
        }
    }
    return steps
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input12.txt".to_string()
    ].iter().collect();

    let mut start: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];

    let mut all_a: Vec<Point> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(row) = line {
                if row.contains('S') { start = (idx, index(row.clone(), 'S').unwrap()) }

                let mut row_chars: Vec<char> = vec![];
                for c in row.chars() { row_chars.push(c); }
                map.push(row_chars);

                for (jdx, _) in row.clone().chars().enumerate().filter(|(_, c)| c == &'a') { all_a.push((idx, jdx)); }
            }
        }
    }

    let s_dist = find_dist(start, &map);
    let mut min_dist: u64 = s_dist;

    for pt in all_a {
        min_dist = min(min_dist, find_dist(pt, &map));
    }

    return (11, s_dist, min_dist)
}
