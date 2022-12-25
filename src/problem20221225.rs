use std::path::PathBuf;

use crate::utils::read_lines;


fn snafu_lookup(numeral: char) -> i64 {
    match numeral {
        '0' => { 0 },
        '1' => { 1 },
        '2' => { 2 },
        '-' => { -1 },
        '=' => { -2 },
        _ => panic!()
    }
}


fn rev_snafu_lookup(num: i64) -> char {
    match num {
        0 => { '0' },
        1 => { '1' },
        2 => { '2' },
        -1 => { '-' },
        -2 => { '=' },
        _ => panic!()
    }
}


fn from_snafu(snafu_num: &String) -> i64 {
    let mut ret: i64 = 0;
    for (idx, c) in snafu_num.chars().rev().enumerate() {
        ret += (5_i64.pow(idx as u32) as i64) * snafu_lookup(c);
    }
    return ret
}


fn to_base5(num: i64) -> Vec<i64> {
    return if num == 0 {
        vec![0]
    } else {
        let mut num = num;
        let mut ret: Vec<i64> = vec![];
        while num > 0 {
            ret.push(num % 5);
            num /= 5;
        }
        ret
    }
}


fn _from_base5(numv: Vec<i64>) -> i64 {
    return numv.iter().zip(0..).map(|(x, y)| x * (5_i32.pow(y) as i64)).sum()
}


fn to_snafu(num: i64) -> String {
    let mut b5: Vec<i64> = to_base5(num);
    for idx in 0..b5.len()-1 {
        while b5[idx] > 2 {
            b5[idx] -= 5;
            b5[idx+1] += 1;
        }
    }

    while b5[b5.len()-1] > 2 {
        b5.push(0);
        let l: usize = b5.len();
        while b5[l-2] > 2 {
            b5[l-2] -= 5;
            b5[l-1] += 1;
        }
    }

    return b5.iter().rev().map(|x|rev_snafu_lookup(*x)).collect();
}


pub fn problem() -> (usize, String, String) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input25.txt".to_string()
    ].iter().collect();

    let mut snafu_sum: i64 = 0;
    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(snafu_num) = line {
                assert_eq!(snafu_num, to_snafu(from_snafu(&snafu_num)));
                snafu_sum += from_snafu(&snafu_num);
            }
        }
    }

    return (24, to_snafu(snafu_sum), "".to_string())
}
