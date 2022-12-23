use std::path::PathBuf;
use std::string::ToString;

use crate::utils::read_lines;


fn process_instructions(concat_instructions: &String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let mut buf: String = "".to_string();

    for c in concat_instructions.chars() {
        if c == 'R' || c == 'L' {
            if buf.len() > 0 {
                ret.push(buf.to_string());
                buf = "".to_string();
            }
            ret.push(c.to_string());
        } else {
            buf.push(c);
        }
    }

    if buf.len() > 0 { ret.push(buf.to_string()); }

    return ret
}


fn turn(dir: char, rot: &String) -> char {
    assert!(rot == "R" || rot == "L");
    match (dir, rot == &"R") {
        ('u', true) => { 'r' },
        ('u', false) => { 'l' },
        ('r', true) => { 'd' },
        ('r', false) => { 'u' },
        ('d', true) => { 'l' },
        ('d', false) => { 'r' },
        ('l', true) => { 'u' },
        ('l', false) => { 'd' },
        _ => panic!(),
    }
}


fn apply_instructions(row: usize,
                      col: usize,
                      map: &Vec<Vec<char>>,
                      dir: char,
                      instructions: &Vec<String>,
                      row_l: &Vec<usize>,
                      row_r:& Vec<usize>,
                      col_t: &Vec<usize>,
                      col_b: &Vec<usize>,
                      stitch: Box<dyn Fn(usize, usize, char) -> (usize, usize, char)>) -> (usize, usize, char) {
    let mut row: usize = row;
    let mut col: usize = col;
    let mut dir: char = dir;

    let mut jp_row: usize;
    let mut jp_col: usize;
    let mut jp_dir: char;

    let mut travel: usize;

    for inst in instructions {
        if inst == &"R".to_string() || inst == &"L".to_string() {
            dir = turn(dir, inst);
        } else {
            travel = inst.parse::<usize>().unwrap();

            for _ in 0..travel {
                match dir {
                    'r' => {
                        if col == row_r[row] {
                            (jp_row, jp_col, jp_dir) = stitch(row, col, dir);
                            if map[jp_row][jp_col] == '#' { break } else { (row, col, dir) = (jp_row, jp_col, jp_dir); }
                        } else if map[row][col + 1] == '#' {
                            break
                        } else {
                            col += 1;
                        }
                    },
                    'd' => {
                        if row == col_b[col] {
                            (jp_row, jp_col, jp_dir) = stitch(row, col, dir);
                            if map[jp_row][jp_col] == '#' { break } else { (row, col, dir) = (jp_row, jp_col, jp_dir); }
                        } else if map[row + 1][col] == '#' {
                            break
                        } else {
                            row += 1;
                        }
                    },
                    'l' => {
                        if col == row_l[row] {
                            (jp_row, jp_col, jp_dir) = stitch(row, col, dir);
                            if map[jp_row][jp_col] == '#' { break } else { (row, col, dir) = (jp_row, jp_col, jp_dir); }
                        } else if map[row][col - 1] == '#' {
                            break
                        } else {
                            col -= 1;
                        }
                    },
                    'u' => {
                        if row == col_t[col] {
                            (jp_row, jp_col, jp_dir) = stitch(row, col, dir);
                            if map[jp_row][jp_col] == '#' { break } else { (row, col, dir) = (jp_row, jp_col, jp_dir); }
                        } else if map[row - 1][col] == '#' {
                            break
                        } else {
                            row -= 1;
                        }
                    },
                    _ => panic!()
                }
            }
        }
    }

    return (row, col, dir)
}


fn transform_result(result: (usize, usize, char)) -> u64 {
    return (1000 * (result.0 + 1) + 4 * (result.1 + 1) + match result.2 {
        'r' => { 0 },
        'd' => { 1 },
        'l' => { 2 },
        'u' => { 3 },
        _ => panic!(),
    }) as u64
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input22.txt".to_string()
    ].iter().collect();

    let mut map_lines: Vec<String> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(input_str) = line {
                map_lines.push(input_str);
            }
        }
    }


    let instructions: Vec<String> = process_instructions(&map_lines.pop().unwrap());
    map_lines.pop().unwrap();

    let max_col: usize = map_lines.iter().map(|x| x.len()).max().unwrap();
    for idx in 0..map_lines.len() {
        let mut line = map_lines[idx].to_string();
        while line.len() < max_col { line.push(' '); }
        map_lines[idx] = line;
    }

    let map: Vec<Vec<char>> = map_lines.iter().map(|line| line.chars().map(|c| c).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let rows: usize = map.len();
    let cols: usize = map[0].len();

    let placeholder: usize = 1000000;
    let mut row_l: Vec<usize> = vec![];
    let mut row_r: Vec<usize> = vec![];
    for _ in 0..rows {
        row_l.push(placeholder);
        row_r.push(placeholder);
    }
    let mut col_t: Vec<usize> = vec![];
    let mut col_b: Vec<usize> = vec![];
    for _ in 0..cols {
        col_t.push(placeholder);
        col_b.push(placeholder);
    }

    for (idx, row) in map.iter().enumerate() {
        for (jdx, c) in row.iter().enumerate() {
            if c != &' ' && row_l[idx] == placeholder { row_l[idx] = jdx; }
            if c != &' ' { row_r[idx] = jdx; }
        }
    }
    for jdx in 0..cols {
        for idx in 0..rows {
            let c: char = map[idx][jdx];
            if c != ' ' && col_t[jdx] == placeholder { col_t[jdx] = idx; }
            if c != ' ' { col_b[jdx] = idx; }
        }
    }

    let srow: usize = 0;
    let scol: usize = map[0].iter().zip(0..cols).filter(|(c, _)| c != &&' ').map(|(_, col)| col).next().unwrap();

    let row_l_clone: Vec<usize> = row_l.clone();
    let row_r_clone: Vec<usize> = row_r.clone();
    let col_t_clone: Vec<usize> = col_t.clone();
    let col_b_clone: Vec<usize> = col_b.clone();

    let wrap = move |row: usize, col: usize, dir: char| -> (usize, usize, char) {
        match dir {
            'r' => { (row, row_l_clone[row], 'r') },
            'd' => { (col_t_clone[col], col, 'd') },
            'l' => { (row, row_r_clone[row], 'l') },
            'u' => { (col_b_clone[col], col, 'u') },
            _ => panic!(),
        }
    };

    fn cube(row: usize, col: usize, dir: char) -> (usize, usize, char) {
        //    ab
        //   f□□c
        //   e□d
        // f □□c
        // a □g
        // b

        // a
        if dir == 'u' && row == 0 && 50 <= col && col < 100 { return (col + 100, 0, 'r') }
        if dir == 'l' && col == 0 && 150 <= row && row < 200 { return (0, row - 100, 'd') }
        // b
        if dir == 'u' && row == 0 && 100 <= col && col < 150 { return (199, col - 100, 'u') }
        if dir == 'd' && row == 199 && col < 50 { return (0, col + 100, 'd') }
        // c
        if dir == 'r' && col == 149 && row < 50 { return (149 - row, 99, 'l') }
        if dir == 'r' && col == 99 && 100 <= row && row < 150 { return (149 - row, 149, 'l') }
        // d
        if dir == 'd' && row == 49 && 100 <= col && col < 150 { return (col - 50, 99, 'l') }
        if dir == 'r' && col == 99 && 50 <= row && row < 100 { return (49, row + 50, 'u') }
        // e
        if dir == 'l' && col == 50 && 50 <= row && row < 100 { return (100, row - 50, 'd') }
        if dir == 'u' && row == 100 && col < 50 { return (col + 50, 50, 'r') }
        // f
        if dir == 'l' && col == 0 && 100 <= row && row < 150 { return (149 - row, 50, 'r') }
        if dir == 'l' && col == 50 && row < 50 { return (149 - row, 0, 'r') }
        // g
        if dir == 'r' && col == 49 && 150 <= row && row < 200 { return (149, row - 100, 'u') }
        if dir == 'd' && row == 149 && 50 <= col && col < 100 { return (col + 100, 49, 'l') }
        panic!("{}", format!("{} {} {}", row, col, dir))
    }

    let result: (usize, usize, char) = apply_instructions(srow,
                                                          scol,
                                                          &map,
                                                          'r',
                                                          &instructions,
                                                          &row_l,
                                                          &row_r,
                                                          &col_t,
                                                          &col_b,
                                                          Box::new(wrap));
    let result_cube: (usize, usize, char) = apply_instructions(srow,
                                                               scol,
                                                               &map,
                                                               'r',
                                                               &instructions,
                                                               &row_l,
                                                               &row_r,
                                                               &col_t,
                                                               &col_b,
                                                               Box::new(cube));

    return (21, transform_result(result), transform_result(result_cube))
}
