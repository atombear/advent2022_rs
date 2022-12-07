use std::path::PathBuf;

use crate::utils::read_lines;

fn get_common(s0: &str, s1: &str) -> char {
    for c in s0.chars() {
        if s1.contains(c) {
            return c
        }
    }
    return '_'
}

fn get_all_common(s0: String, s1: String) -> String {
    let mut ret: String = "".to_string();
    for c0 in s0.chars() {
        if s1.contains(c0) {
            ret.push(c0);
        }
    }
    return ret
}

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_score(c: char) -> u64 {
    for (idx, c_other) in LETTERS.chars().enumerate() {
        if c == c_other {
            return (idx + 1) as u64
        }
    }
    return 0
}

fn get_common_three(s0: String, s1: String, s2: String) -> char {
    get_all_common(s0, get_all_common(s1, s2)).chars().nth(0).unwrap()
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input3.txt".to_string()
    ].iter().collect();

    let mut score = 0;

    let mut group_score: u64 = 0;
    let mut words: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string()];

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(bag) = line {

                let len = bag.len() / 2;
                let s0 = &bag[0..len];
                let s1 = &bag[len..];

                // first part
                score += get_score(get_common(s0, s1));

                // second part
                words[idx % 3] = bag.to_string();

                if idx % 3 == 2 {
                    group_score += get_score(get_common_three(words[0].to_string(),
                                                              words[1].to_string(),
                                                              words[2].to_string()));
                }
            }
        }
    }

    return (2, score, group_score)
}
