use std::cmp::max;
use std::path::PathBuf;

use crate::utils::read_lines;

fn rot90<T: Copy>(arr: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut arr = arr;
    arr.reverse();
    return (0..arr[0].len()).map(|jdx| arr.iter().map(|row| row[jdx]).collect()).collect();
}


// brutal accounting...
fn get_scenic_score(trees: &Vec<Vec<u64>>, idx: usize, jdx: usize) -> u64 {
    let mut scenic_score: u64 = 1;
    let rows = trees.len();
    let cols = trees[0].len();

    let h = trees[idx][jdx];

    let mut delta: u64;
    let mut idx_delta: usize;

    if idx != 0 && idx != rows-1 && jdx != 0 && jdx != rows -1 {

        idx_delta = idx + 1;
        while idx_delta < rows && trees[idx_delta][jdx] < h { idx_delta+=1; }
        scenic_score *= (idx_delta - idx - 1) as u64;

        idx_delta = jdx + 1;
        while idx_delta < cols && trees[idx][idx_delta] < h { idx_delta+=1; }
        scenic_score *= (idx_delta - jdx - 1) as u64;

        delta = 1;
        while trees[idx - delta as usize][jdx] < h {
            delta += 1;
            if idx == (delta-1) as usize {
                delta -= 1;
                break
            }
        }
        scenic_score *= delta;

        delta = 1;
        while trees[idx][jdx - delta as usize] < h {
            delta += 1;
            if jdx == (delta-1) as usize {
                delta -= 1;
                break;
            }
        }
        scenic_score *= delta;

    } else {
        scenic_score = 0;
    }

    return scenic_score;
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input8.txt".to_string()
    ].iter().collect();

    let mut trees: Vec<Vec<u64>> = vec![];
    let mut bit_trees: Vec<Vec<bool>> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(tree_row_str) = line {
                let mut tree_row: Vec<u64> = vec![];
                let mut bit_row: Vec<bool> = vec![];
                for c in tree_row_str.chars() {
                    tree_row.push(c.to_string().parse::<u64>().unwrap());
                    bit_row.push(false);
                }
                trees.push(tree_row);
                bit_trees.push(bit_row);
            }
        }
    }

    let mut max_tree: u64;
    for _ in 0..4 {
        for (idx, row) in trees.iter().enumerate() {
            max_tree = 0;
            for (jdx, h) in row.iter().enumerate() {
                if *h > max_tree || jdx == 0 {
                    bit_trees[idx][jdx] = true;
                }
                max_tree = max(max_tree, *h);
            }
        }
        trees = rot90(trees);
        bit_trees = rot90(bit_trees);
    }

    let visible_trees: u64 = bit_trees.iter().map(|row| row.iter().map(|tree| if *tree {1} else {0}).sum::<u64>()).sum();

    let rows = trees.len();
    let cols = trees[0].len();
    let mut max_scenic_score: u64 = 0;

    for idx in 0..rows {
        for jdx in 0..cols {
            if bit_trees[idx][jdx] {
                max_scenic_score = max(max_scenic_score, get_scenic_score(&trees, idx, jdx));
            }
        }
    }
    return (7, visible_trees, max_scenic_score)
}
