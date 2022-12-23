use std::path::PathBuf;

use std::collections::HashSet;

use crate::utils::read_lines;

fn sgn(v: i64) -> i64 {
    return if v > 0 { 1 } else { -1 }
}

type Point = (i64, i64);
const BOX_DELTA: [Point; 9] = [(0,0), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)];
fn resolve_follower(loc_h: Point, loc_t: Point) -> Point {
    let xh;
    let yh;
    (xh, yh) = loc_h;

    let mut xt;
    let mut yt;
    (xt, yt) = loc_t;

    if BOX_DELTA.map(|(xd, yd)| (xh+xd, yh+yd)).contains(&(xt, yt)) {
        // no change
    } else if xh == xt {
        yt = yh + sgn(yt - yh);
    } else if yh == yt {
        xt = xh + sgn(xt - xh);
    } else {
        xt += sgn(xh - xt);
        yt += sgn(yh - yt);
    }

    return (xt, yt)
}


fn mv_head(loc_h: Point, dir: &str) -> Point {
    let mut xh;
    let mut yh;
    (xh, yh) = loc_h;
    if dir == "R" { xh += 1; } else if dir == "L" { xh -= 1; } else if dir == "U" { yh += 1; } else if dir == "D" { yh -= 1; }
    else { panic!("{}", format!("unknown char {}!", dir)); }
    return (xh, yh)
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input9.txt".to_string()
    ].iter().collect();

    let mut dir: &str;
    let mut dist: u64;

    let mut locs: HashSet<Point> = HashSet::new();

    let mut loc_h: Point = (0, 0);
    let mut loc_t: Point = (0, 0);
    locs.insert(loc_h);


    let rope_size: usize = 10;
    let mut rope: Vec<Point> = vec![];
    for _ in 0..rope_size {
        rope.push((0,0));
    }

    let mut rope_locs: HashSet<Point> = HashSet::new();
    rope_locs.insert((0, 0));

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(mv_instr) = line {
                let dir_dist: Vec<&str> = mv_instr.split(" ").collect();

                dir = dir_dist[0];
                dist = dir_dist[1].parse::<u64>().unwrap();

                for _ in 0..dist {
                    loc_h = mv_head(loc_h, dir);
                    loc_t = resolve_follower(loc_h, loc_t);
                    locs.insert(loc_t);
                }

                for _ in 0..dist {
                    rope[0] = mv_head(rope[0], dir);

                    for idx in 1..rope_size {
                        rope[idx] = resolve_follower(rope[idx-1], rope[idx]);
                    }
                    rope_locs.insert(rope[rope_size-1]);
                }
            }
        }
    }

    return (8, locs.len() as u64, rope_locs.len() as u64)
}
