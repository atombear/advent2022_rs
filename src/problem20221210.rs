use std::path::PathBuf;

use crate::utils::read_lines;

struct CycleParams {
    num: i64,
    sum: i64,
    signal: i64,
}


fn complete_cycle(cycle_params: &mut CycleParams,
                  val: i64,
                  crt: String) -> String {

    let pixel: i64 = (cycle_params.num - 1) % 40;
    let mut crt = crt;

    crt.push(
        if [-1, 1, 0].contains(&(pixel - cycle_params.sum)) { '#' } else { '.' }
    );

    cycle_params.num += 1;
    cycle_params.sum += val;

    if [20, 60, 100, 140, 180, 220].contains(&cycle_params.num) {
        cycle_params.signal += cycle_params.num * cycle_params.sum;
    }

    return crt
}


pub fn problem() -> (usize, i64, String) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input10.txt".to_string()
    ].iter().collect();

    let mut cycle_params: CycleParams = CycleParams { sum: 1, signal: 0, num: 1};

    let mut val: i64;
    let mut crt: String = "".to_string();

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(op) = line {
                crt = complete_cycle(&mut cycle_params, 0, crt);

                if op != "noop" {
                    assert!(op.contains("addx"));
                    val = op.split(' ').nth(1).unwrap().parse::<i64>().unwrap();
                    crt = complete_cycle(&mut cycle_params, val, crt);
                }
            }
        }
    }

    let image = (0..6).map(|x| &crt[(x*40)..((1 + x)*40)])
                             .fold("".to_string(), |x, y| format!("{}\n{}", x, y));
    return (9, cycle_params.signal, image)
}
