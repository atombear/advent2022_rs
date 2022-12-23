use std::cmp::max;
use std::path::PathBuf;
use std::collections::HashSet;

use crate::utils::read_lines;


type Point = (i64, i64);
type Piece = HashSet<Point>;


fn get_next_direction(wind_pattern: &Vec<i64>, idx: &mut usize) -> i64 {
    let ret = wind_pattern[*idx % wind_pattern.len()];
    *idx += 1;
    return ret
}


fn min_x(piece: &Piece) -> i64 { piece.iter().map(|(x, _y)| *x).min().unwrap() }
fn max_x(piece: &Piece) -> i64 { piece.iter().map(|(x, _y)| *x).max().unwrap() }
fn max_y(piece: &Piece) -> i64 { piece.iter().map(|(_x, y)| *y).max().unwrap() }


fn shift_lr(piece: &Piece, delta: i64) -> Piece {
    let delta: i64 = if delta + min_x(piece) < 0 || delta + max_x(piece) > 6 { 0 } else { delta };
    return piece.iter().map(|(x, y)| (x + delta, *y)).collect();
}


fn shift_ud(piece: &Piece, delta: i64) -> Piece {
    return piece.iter().map(|(x, y)| (*x, y + delta)).collect();
}


fn basic_sim(wind_pattern: &Vec<i64>, steps: i64) -> (u64, Vec<(usize, i64, i64)>) {
    // empirically determined, but could be automated by identifying the first wind_num above the
    // periodicity
    let offset: usize = match wind_pattern.len() {
        40 => 3,
        10091 => 1,
        _ => panic!("not supported"),
    };

    let mut periodicity_table: Vec<(usize, i64,i64)> = vec![];
    let periodicity: usize = 5 * wind_pattern.len();

    let piece0: Piece = HashSet::from([(0,0), (1,0), (2,0), (3,0)]);
    let piece1: Piece = HashSet::from([(1,0), (0,1), (1,1), (2,1), (1,2)]);
    let piece2: Piece = HashSet::from([(0,0), (1,0), (2,0), (2,1), (2,2)]);
    let piece3: Piece = HashSet::from([(0,0), (0,1), (0,2), (0,3)]);
    let piece4: Piece = HashSet::from([(0,0), (0,1), (1,0), (1,1)]);

    // run the simulation to fill the periodicity table
    let mut all_spots: HashSet<Point> = HashSet::new();
    // add a floor
    for x in 0..7 { all_spots.insert((x, -1)); }

    let mut piece: Piece;
    let mut proposed_piece: Piece;

    let mut height: i64 = -1;
    let mut wind_num: usize = 0;

    for piece_num in 0i64..steps {

        if wind_num >= periodicity + offset && wind_num <= 2 * periodicity + offset {
            periodicity_table.push((wind_num, height, piece_num));
        }

        // get piece
        piece = match piece_num % 5 {
            0 => piece0.clone(),
            1 => piece1.clone(),
            2 => piece2.clone(),
            3 => piece3.clone(),
            4 => piece4.clone(),
            _ => panic!("this is impossible"),
        };

        piece = shift_ud(&shift_lr(&piece, 2), 4 + height);

        loop {
            // shift left / right
            proposed_piece = shift_lr(&piece, get_next_direction(&wind_pattern, &mut wind_num));
            match proposed_piece.intersection(&all_spots).next() {
                None => { piece = proposed_piece; },
                _ => {},
            }

            // drop
            proposed_piece = shift_ud(&piece, -1);
            match proposed_piece.intersection(&all_spots).next() {
                None => { piece = proposed_piece; },
                _ => { break },
            }
        }
        // absorb piece into structure
        for p in piece.iter() { all_spots.insert(*p); }
        // update height of tower
        height = max(max_y(&piece), height);
    }

    return (1 + height as u64, periodicity_table)
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input17.txt".to_string()
    ].iter().collect();

    let mut wind_pattern: Vec<i64> = vec![];
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(wind_str) = line {
                wind_pattern = wind_str.chars().map(|c| if c == '>' { 1 } else { -1 }).collect();
            }
        }
    }

    let periodicity_table = basic_sim(&wind_pattern, 20000).1;

    let num_el: usize = periodicity_table.len() - 1;
    assert_eq!(periodicity_table[num_el].0 - periodicity_table[0].0, 5 * wind_pattern.len());

    // use the periodicity table to determine how many pieces are added and what height is gained
    // each period
    let mut num_pieces: i64 = periodicity_table[num_el].2;
    let delta_pieces: i64 = periodicity_table[num_el].2 - periodicity_table[0].2;

    let mut height: i64 = periodicity_table[num_el].1;
    let delta_height: i64 = periodicity_table[num_el].1 - periodicity_table[0].1;

    // run that up as far as possible
    while num_pieces < 1000000000000 {
        height += delta_height;
        num_pieces += delta_pieces;
    }
    height -= delta_height;
    num_pieces -= delta_pieces;

    // use the periodicity table to determine the remainder
    let pieces_remaining: i64 = 1000000000000 - num_pieces;
    height += periodicity_table[pieces_remaining as usize].1 - periodicity_table[0].1;

    return (16, basic_sim(&wind_pattern, 2022).0, 1 + height as u64)
}
