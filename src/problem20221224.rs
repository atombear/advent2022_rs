use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::utils::read_lines;


type Point = (i64, i64);


fn update_blizzards(blizz: &HashSet<Point>, drow: i64, dcol: i64, rows: usize, cols: usize) -> HashSet<Point> {
    let rows: i64 = rows as i64;
    let cols: i64 = cols as i64;
    blizz.iter().map(|(r, c)| (((r-1+drow).rem_euclid(rows-2)) + 1, ((c-1+dcol).rem_euclid(cols-2)) + 1)).collect()
}


fn prime_factors(num: u64) -> HashMap<u64, u64> {
    let mut num: u64 = num;
    let mut ret: HashMap<u64, u64> = HashMap::new();
    let mut mult: u64 = 2;
    while num != 1 {
        while num % mult == 0 {
            if !ret.contains_key(&mult) { ret.insert(mult, 0); };
            *ret.get_mut(&mult).unwrap() += 1;
            num /= mult;
        }
        mult += 1;
    }
    return ret
}


fn find_lcm(num0: u64, num1: u64) -> u64 {
    let factors0: HashMap<u64, u64> = prime_factors(num0);
    let factors1: HashMap<u64, u64> = prime_factors(num1);

    let mut factors: HashSet<u64> = HashSet::new();
    factors.extend(factors0.keys().chain(factors1.keys()));

    let mut ret: u64 = 1;
    for factor in factors {
        for _ in 0..max(*factors0.get(&factor).unwrap_or(&0),*factors1.get(&factor).unwrap_or(&0)) {
            ret *= factor;
        }
    }

    return ret
}


fn get_adj(exp: &Point,
           start: &Point,
           end: &Point,
           r_blizz: &HashSet<Point>,
           l_blizz: &HashSet<Point>,
           u_blizz: &HashSet<Point>,
           d_blizz: &HashSet<Point>,
           rows: usize,
           cols: usize,
           mod_time: u64,
           visited: &mut HashSet<(u64, Point)>) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];
    let mut new_exp: Point;
    for (drow, dcol) in [(0,0),(1,0),(-1,0),(0,1),(0,-1)] {
        new_exp = (exp.0 + drow, exp.1 + dcol);
        if !visited.contains(&(mod_time, new_exp)) &&
            (
                (new_exp == *start) ||
                (new_exp == *end) ||
                (
                    (0 < new_exp.0 && (new_exp.0 as usize) < rows-1 && 0 < new_exp.1 && (new_exp.1 as usize) < cols-1) &&
                    ![r_blizz, l_blizz, u_blizz, d_blizz].iter().map(|x| x.contains(&new_exp)).any(|x| x)
                )
            ) {
            ret.push(new_exp);
            visited.insert((mod_time, new_exp));
        }
    }
    return ret
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input24.txt".to_string()
    ].iter().collect();

    let mut l_blizz: HashSet<Point> = HashSet::new();
    let mut r_blizz: HashSet<Point> = HashSet::new();
    let mut u_blizz: HashSet<Point> = HashSet::new();
    let mut d_blizz: HashSet<Point> = HashSet::new();

    let mut cols: usize = 0;
    let mut rows: usize = 0;

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            rows = idx;
            if let Ok(map_line) = line {
                cols = map_line.len();

                for (jdx, c) in map_line.chars().enumerate() {
                    match c {
                        'v' => { d_blizz.insert((idx as i64, jdx as i64)); },
                        '>' => { r_blizz.insert((idx as i64, jdx as i64)); },
                        '<' => { l_blizz.insert((idx as i64, jdx as i64)); },
                        '^' => { u_blizz.insert((idx as i64, jdx as i64)); },
                        '#' => {},
                        '.' => {},
                        _ => panic!()
                    }
                }
            }
        }
    }
    rows += 1;

    let start_pt: Point = (0, 1);
    let end_pt: Point = ((rows - 1) as i64, (cols-2) as i64);
    let mut time0: u64 = 0;

    let mut time: u64 = 0;
    // lcm of rows/cols for the input data which is 25 x 120
    let lcm: u64 = find_lcm(rows as u64 - 2, cols as u64 - 2);

    for idx in 0..3 {
        let start: Point = if idx % 2 == 0 { start_pt } else { end_pt };
        let end: Point = if idx % 2 == 1 { start_pt } else { end_pt };

        let mut spread: Vec<Point> = vec![start];
        let mut next_spread: Vec<Point> = vec![];
        let mut visited: HashSet<(u64, Point)> = HashSet::from([(time, start)]);
        let mut solved: bool = false;
        while !solved {
            time += 1;
            r_blizz = update_blizzards(&r_blizz, 0, 1, rows, cols);
            l_blizz = update_blizzards(&l_blizz, 0, -1, rows, cols);
            u_blizz = update_blizzards(&u_blizz, -1, 0, rows, cols);
            d_blizz = update_blizzards(&d_blizz, 1, 0, rows, cols);

            while let Some(exp) = spread.pop() {
                for next_point in get_adj(&exp, &start, &end, &r_blizz, &l_blizz, &u_blizz, &d_blizz, rows, cols, time % lcm, &mut visited) {
                    if next_point == end {
                        solved = true;
                        if idx == 0 { time0 = time; }
                    }

                    next_spread.push(next_point);
                }
            }
            while let Some(exp) = next_spread.pop() { spread.push(exp); }
        }
    }

    return (23, time0, time)
}
