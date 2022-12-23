use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::utils::read_lines;


type Point = (i64, i64);


fn check_north(elves: &HashSet<Point>, elf: &Point) -> bool {
    let x: i64;
    let y: i64;
    (x, y) = *elf;
    return !elves.contains(&(x-1, y+1)) && !elves.contains(&(x, y+1)) && !elves.contains(&(x+1, y+1))
}
fn north(elf: &Point) -> Point { (elf.0, elf.1+1) }


fn check_south(elves: &HashSet<Point>, elf: &Point) -> bool {
    let x: i64;
    let y: i64;
    (x, y) = *elf;
    return !elves.contains(&(x-1, y-1)) && !elves.contains(&(x, y-1)) && !elves.contains(&(x+1, y-1))
}
fn south(elf: &Point) -> Point { (elf.0, elf.1-1) }


fn check_west(elves: &HashSet<Point>, elf: &Point) -> bool {
    let x: i64;
    let y: i64;
    (x, y) = *elf;
    return !elves.contains(&(x-1, y-1)) && !elves.contains(&(x-1, y)) && !elves.contains(&(x-1, y+1))
}
fn west(elf: &Point) -> Point { (elf.0-1, elf.1) }


fn check_east(elves: &HashSet<Point>, elf: &Point) -> bool {
    let x: i64;
    let y: i64;
    (x, y) = *elf;
    return !elves.contains(&(x+1, y-1)) && !elves.contains(&(x+1, y)) && !elves.contains(&(x+1, y+1))
}
fn east(elf: &Point) -> Point { (elf.0+1, elf.1) }


const CHECK: [fn(&HashSet<Point>, &Point) -> bool; 4] = [check_north, check_south, check_west, check_east];
const MOVE: [fn(&Point) -> Point; 4] = [north, south, west, east];


fn all_clear(elves: &HashSet<Point>, elf: &Point) -> bool {
    let x: i64;
    let y: i64;
    (x, y) = *elf;
    for (dx, dy) in [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
        if elves.contains(&(x+dx, y+dy)) { return false }
    }
    return true
}

fn propose(elves: &HashSet<Point>, proposal: &mut HashMap<Point, Vec<Point>>, cycle: usize) {
    let mut new_point: Point = (1000000, 1000000);
    let mut can_move: bool;
    let mut cycle_idx: usize;

    for elf in elves {
        can_move = false;
        if !all_clear(elves, elf) {
            for idx in 0..4 {
                cycle_idx = (idx + cycle) % 4;
                if (CHECK[cycle_idx])(elves, elf) {
                    new_point = (MOVE[cycle_idx])(elf);
                    can_move = true;
                    break
                }
            }
        }

        if can_move {
            if proposal.contains_key(&new_point) {
                proposal.get_mut(&new_point).unwrap().push(*elf)
            } else {
                proposal.insert(new_point, vec![*elf]);
            }
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
        for (jdx, c) in line.chars().enumerate().filter(|(_, c)| c == &'#') {
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
