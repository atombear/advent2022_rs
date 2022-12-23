use std::path::PathBuf;

use crate::utils::read_lines;


fn get_bounds(s: &str) -> (u64, u64) {
    let bounds: Vec<&str> = s.split('-').collect();
    return (bounds[0].parse::<u64>().unwrap(), bounds[1].parse::<u64>().unwrap())
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input4.txt".to_string()
    ].iter().collect();

    let mut redundant: u64 = 0;
    let mut more_redundant: u64 = 0;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(cleaning_schedule) = line {
                let pair: Vec<&str> = cleaning_schedule.split(',').collect();
                let (x0, x1) = get_bounds(pair[0]);
                let (y0, y1) = get_bounds(pair[1]);

                if x0 <= y0 && x1 >= y1 {
                    redundant += 1;
                    more_redundant += 1;
                } else if y0 <= x0 && y1 >= x1 {
                    redundant += 1;
                    more_redundant += 1;
                } else if !(x1 < y0 || y1 < x0) {
                    more_redundant += 1;
                }
            }
        }
    }

    return (3, redundant, more_redundant)
}
