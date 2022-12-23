use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::slice::Iter;

use crate::utils::read_lines;


type Point = (i64, i64);


fn check_points(elves: &HashSet<Point>, elf: &Point, iter: &mut Iter<(i64, i64)>) -> bool {
    !(iter.any(|(dx, dy)| elves.contains(&(elf.0+dx, elf.1+dy))))
}


fn check_north(elves: &HashSet<Point>, elf: &Point) -> bool { check_points(elves, elf,&mut [(-1,1),(0,1),(1,1)].iter()) }
fn north(elf: &Point) -> Point { (elf.0, elf.1+1) }


fn check_south(elves: &HashSet<Point>, elf: &Point) -> bool { check_points(elves, elf, &mut [(-1,-1),(0,-1),(1,-1)].iter()) }
fn south(elf: &Point) -> Point { (elf.0, elf.1-1) }


fn check_west(elves: &HashSet<Point>, elf: &Point) -> bool { check_points(elves, elf, &mut [(-1,-1),(-1,0),(-1,1)].iter()) }
fn west(elf: &Point) -> Point { (elf.0-1, elf.1) }


fn check_east(elves: &HashSet<Point>, elf: &Point) -> bool { check_points(elves, elf, &mut [(1,-1),(1,0),(1,1)].iter()) }
fn east(elf: &Point) -> Point { (elf.0+1, elf.1) }


const CHECK: [fn(&HashSet<Point>, &Point) -> bool; 4] = [check_north, check_south, check_west, check_east];
const MOVE: [fn(&Point) -> Point; 4] = [north, south, west, east];
const SURROUNDING_DELTAS: [(i64, i64); 8] = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];


fn all_clear(elves: &HashSet<Point>, elf: &Point) -> bool {
    return SURROUNDING_DELTAS
        .iter()
        .all(|(dx, dy)| !elves.contains(&(elf.0+dx, elf.1+dy)))
}

fn propose(elves: &HashSet<Point>, proposal: &mut HashMap<Point, Vec<Point>>, cycle: usize) {
    let default_state: (bool, Point) = (false, (1000000, 1000000));
    let mut new_point: Point;
    let mut can_move: bool;

    for elf in elves {
        (can_move, new_point) = if all_clear(elves, elf) {
            default_state
        } else {
            (0..4)
                .map(|idx|
                ((CHECK[(idx + cycle) % 4])(elves, elf),
                 (MOVE[(idx + cycle) % 4])(elf)))
                .filter(|(check, _)| *check)
                .next()
                .unwrap_or(default_state)
        };

        if can_move {
            if !proposal.contains_key(&new_point) { proposal.insert(new_point, vec![]); }
            proposal.get_mut(&new_point).unwrap().push(*elf)
        }
    }
}


fn update(elves: &mut HashSet<Point>, proposal: &mut HashMap<Point, Vec<Point>>) {
    for (move_to, move_from) in proposal.drain().filter(|(_, move_from)| move_from.len() == 1) {
        elves.remove(&move_from[0]);
        elves.insert(move_to);
    }
    assert!(proposal.is_empty())
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input23.txt".to_string()
    ].iter().collect();

    let mut all_lines: Vec<String> = vec![];
    let mut elves: HashSet<Point> = HashSet::new();
    let mut proposal: HashMap<Point, Vec<Point>> = HashMap::new();

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(elf_line) = line {
                all_lines.push(elf_line);
            }
        }
    }

    all_lines.reverse();
    for (idx, line) in all_lines.iter().enumerate() {
        for jdx in line.chars().enumerate().filter(|(_, c)| c == &'#').map(|(jdx, _)| jdx) {
            elves.insert((jdx as i64, idx as i64));
        }
    }


    let mut empty: i64 = 0;
    let mut round: usize = 0;
    loop {
        if round == 10 {
            let minx: i64 = elves.iter().map(|p| p.0).min().unwrap();
            let maxx: i64 = elves.iter().map(|p| p.0).max().unwrap();
            let miny: i64 = elves.iter().map(|p| p.1).min().unwrap();
            let maxy: i64 = elves.iter().map(|p| p.1).max().unwrap();
            empty = (1 + maxx - minx) * (1 + maxy - miny) - elves.len() as i64;
        }
        propose(&elves, &mut proposal, round);
        if proposal.len() == 0 { break }
        update(&mut elves, &mut proposal);
        assert!(proposal.is_empty());
        round += 1;
    }

    return (22, empty as u64, 1 + round as u64)
}
