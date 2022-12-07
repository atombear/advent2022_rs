use std::path::PathBuf;

use crate::utils::read_lines;

fn push_circle<T>(buf: &mut Vec<T>, val: T, max_len: usize) {
    buf.insert(0, val);
    if buf.len() > max_len {
        buf.pop();
    }
}

fn has_duplicates<T: std::cmp::Ord + std::clone::Clone>(buf: &Vec<T>) -> bool {
    let mut cp: Vec<T> = buf.to_vec();
    cp.sort();
    for idx in 0..(cp.len() - 1) {
        if cp[idx] == cp[idx+1] {
            return true
        }
    }
    return false
}

pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input6.txt".to_string()
    ].iter().collect();

    let mut buf4: Vec<char> = vec![];
    let mut buf14: Vec<char> = vec![];
    let mut results: [u64; 2] = [0, 0];

    let mut buf_data: [(&mut Vec<char>, usize, usize); 2] = [(&mut buf4, 4, 0), (&mut buf14, 14, 1)];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(msg) = line {
                for (idx, c) in msg.chars().enumerate() {

                    for (buf, max_len, rdx) in &mut buf_data {
                        push_circle(buf, c, *max_len);
                        if !has_duplicates(&buf) && buf.len() == *max_len && results[*rdx] == 0 {
                            results[*rdx] = (idx+1) as u64;
                        }
                    }
                }
            }
        }
    }

    return (5, results[0], results[1])
}
