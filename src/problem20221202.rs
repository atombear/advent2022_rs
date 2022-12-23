use std::path::PathBuf;

use crate::utils::read_lines;


// A - rock         1
// B - paper        2
// C - scissors     3

// X - rock
// Y - paper
// Z - scissors

fn get_score0(p0: char, p1: char) -> u64 {
    match (p0, p1) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => panic!(),
    }
}


// A - rock         1
// B - paper        2
// C - scissors     3

// X - lose
// Y - draw
// Z - win

fn get_score1(p0: char, p1: char) -> u64 {
    match (p0, p1) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        _ => panic!(),
    }
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input2.txt".to_string()
    ].iter().collect();

    let mut score0: u64 = 0;
    let mut score1: u64 = 0;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(game) = line {

                let p0 = game.chars().nth(0).unwrap();
                let p1 = game.chars().nth(2).unwrap();

                score0 += get_score0(p0, p1);

                score1 += get_score1(p0, p1);
            }
        }
    }

    return (1, score0, score1)
}
