use std::path::PathBuf;

use crate::utils::read_lines;


fn confirm_labels(labels: String) {
    assert_eq!(0, (labels.len() - 3) % 4);
    assert!((labels.len() - 3) / 4 < 9);

    let mut assumed_labels = "123456789".chars();
    let mut next_label: char;
    for c in labels.chars() {
        if c != ' ' {
            next_label = assumed_labels.next().unwrap();
            assert_eq!(next_label, c);
        }
    }
}


fn process_arrangement(arr: Vec<String>) -> Vec<Vec<char>> {
    let mut arr = arr;
    arr.reverse();
    let len = arr.len();

    // first element is the empty string
    // the next element is the labelling of each box.
    let labels = &arr[1];
    confirm_labels(labels.to_owned());
    let num_elements = 1 + ((labels.len() - 3) / 4);

    let mut ret = vec![];
    for _ in 0..num_elements { ret.push(vec![]); }

    let mut r: usize;
    let mut c: char;
    for idx in 2..len {
        r = 1;
        for jdx in 0..num_elements {
            c = arr[idx].chars().nth(r).unwrap();
            if c != ' ' {
                ret[jdx].push(c);
            }
            r += 4;
        }
    }
    return ret
}


fn parse_move(m: &String) -> (u64, u64, u64) {
    let words: Vec<&str> = m.split(' ').collect();
    let nums: [u64; 3] = [1,3,5].map(|x| words[x]).map(|x| x.parse::<u64>().unwrap());
    return (nums[0], nums[1], nums[2])
}


fn perform_move(mv: (u64, u64, u64), stacking: &mut Vec<Vec<char>>) {
    let (num, from, to) = mv;
    for _ in 0..num {
        let val = stacking[(from-1) as usize].pop().unwrap();
        stacking[(to-1) as usize].push(val);
    }
}


fn perform_move9001(mv: (u64, u64, u64), stacking: &mut Vec<Vec<char>>) {
    let (num, from, to) = mv;
    let mut cache = 1;
    loop { if cache != from && cache != to { break } else { cache += 1; } }
    perform_move((num, from, cache), stacking);
    perform_move((num, cache, to), stacking);
}


pub fn problem() -> (usize, String, String) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input5.txt".to_string()
    ].iter().collect();

    let mut initial_stacking: Vec<String> = vec![];
    let mut stacking: Vec<Vec<char>> = vec![];
    let mut stacking9001: Vec<Vec<char>> = vec![];

    let mut loading_arrangement: bool = true;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(move_instr) = line {
                if loading_arrangement {
                    initial_stacking.push(move_instr.to_owned());
                } else {
                    let mv = parse_move(&move_instr);
                    perform_move(mv, &mut stacking);
                    perform_move9001(mv, &mut stacking9001);
                }

                if move_instr == "".to_string() {
                    loading_arrangement = false;
                    stacking = process_arrangement(initial_stacking.to_owned());
                    stacking9001 = stacking.to_owned();
                }
            }
        }
    }

    let get_last_elements = |st: Vec<Vec<char>>| -> String {
        st.iter().map(|x| x[x.len() - 1]).collect()
    };

    return (4, get_last_elements(stacking), get_last_elements(stacking9001))
}
