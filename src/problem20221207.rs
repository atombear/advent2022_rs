use std::path::PathBuf;
use std::collections::HashMap;

use fancy_regex::Regex;

use crate::utils::read_lines;

// state machine
#[derive(PartialEq)]
enum ReadState {
    CD,
    LS,
}


// for a list of objects in a directory, the files start with their size
// this only accounts for the size of files in a directory, not including
// the files contained in sub directories
fn get_dir_size(dirlist: &Vec<String>) -> u64 {
    let mut directory_size: u64 = 0;

    // capture digits
    let re_dir_size = Regex::new(r"\d+").unwrap();
    for obj in dirlist {
        if let Ok(Some(size)) = re_dir_size.captures(&obj) {
            if let Ok(val) = size.get(0).unwrap().as_str().parse::<u64>() {
                directory_size += val;
            }
        }
    }
    return directory_size
}


// all the strings up to a delimiter, each time that delimiter
// appears in the original string
fn split_include(s: String, delim: char) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let mut cache: String = "".to_string();
    for c in s.chars() {
        cache.push(c);
        if c == delim { ret.push(cache.to_string()); }
    }
    return ret
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input7.txt".to_string()
    ].iter().collect();

    let mut cwd: String = "".to_string();
    let mut read_state: ReadState = ReadState::CD;
    let mut dir: &str;
    let mut dirlist: Vec<String> = vec![];
    let mut data: Vec<(String, u64)> = vec![];

    let re_dir = Regex::new(r"(?<=\$ cd ).+").unwrap();
    let re_ls = Regex::new(r"\$ ls").unwrap();

    if let Ok(lines) = read_lines(data_path) {
        // parsing the lines as a state machine, where if the last shell command is
        // ls, the succeeding lines, presumed to be the contents of the directory,
        // will be acquired until the next cd command.
        for line in lines {
            if let Ok(shell_cmd) = line {

                if let Ok(Some(cd_cmd)) = re_dir.captures(&shell_cmd) {
                    if dirlist.len() > 0 {
                        data.push((cwd.to_string(), get_dir_size(&dirlist)));
                    }
                    dirlist = vec![];
                    read_state = ReadState::CD;

                    dir = cd_cmd.get(0).unwrap().as_str();
                    if dir == ".." {
                        let depth: usize = cwd.chars().map(|x| if x == '/' { 1 } else { 0 }).sum();
                        cwd = cwd.split('/').map(|x| x.to_string()).collect::<Vec<String>>()[0..depth - 1].join("/");
                        cwd.push('/');
                    } else {
                        cwd.push_str(&dir);
                        if !(dir == "/".to_string()) { cwd.push('/'); }
                    }
                }

                if read_state == ReadState::LS { dirlist.push(shell_cmd.to_string()); }
                if let Ok(Some(_)) = re_ls.captures(&shell_cmd) { read_state = ReadState::LS; }
            }
        }
        // capture the last ls
        if dirlist.len() > 0 {
            data.push((cwd.to_string(), get_dir_size(&dirlist)));
        }
    }

    // sort according to the path.
    data.sort();

    // store the cumulative size of each directory.
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();

    for (dir, size) in data {
        // add the size of each leaf to its parents (including itself).
        for parent in split_include(dir, '/') {
            if let Some(dir_size) = dir_sizes.get_mut(&parent) {
                *dir_size += size;
            } else {
                dir_sizes.insert(parent, size);
            }
        }
    }

    let cum_smallest_directory_sizes: u64 = dir_sizes.values().filter(|x| **x < 100000).sum();

    let necessary_space: u64 = 30000000 - (70000000 - *dir_sizes.get("/").unwrap());
    let smallest_free_dir_size: u64 = *dir_sizes.values().filter(|x| **x >= necessary_space).min().unwrap();

    return (6, cum_smallest_directory_sizes, smallest_free_dir_size)
}
