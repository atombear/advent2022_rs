use std::cmp::{max};
use std::path::PathBuf;

use crate::utils::read_lines;


fn update_triple(triple: &mut Vec<u64>, bag: u64) {
    if bag >= triple[0] {
        triple[2] = triple[1];
        triple[1] = triple[0];
        triple[0] = bag;
    } else if bag >= triple[1] {
        triple[2] = triple[1];
        triple[1] = bag;
    } else if bag >= triple[2] {
        triple[2] = bag;
    }
}

pub fn problem() -> (u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input1.txt".to_string()
    ].iter().collect();

    let mut max_calories: u64 = 0;
    let mut max_triple: Vec<u64> = vec![0,0,0];
    let mut bag: u64 = 0;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(num_str) = line {
                if num_str == "" {
                    update_triple(&mut max_triple, bag);
                    max_calories = max(max_calories, bag);
                    bag = 0;
                } else {
                    bag += num_str.parse::<u64>().unwrap();
                }
            }
        }
    }
    update_triple(&mut max_triple, bag);
    max_calories = max(max_calories, bag);
    return (max_calories, max_triple.iter().sum())
}
