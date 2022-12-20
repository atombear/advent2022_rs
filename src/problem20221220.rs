use std::cell::RefCell;
use std::cmp::{max, min};
use std::path::PathBuf;
use std::rc::Rc;

use crate::utils::read_lines;


struct Order {
    val: i64,
    current_position: usize,
}


fn mix(all_nums: &Vec<i64>, zero_loc: usize, iter: u64, mult: i64) -> i64 {

    let mut original_order: Vec<Rc<RefCell<Order>>> = vec![];
    let mut new_order: Vec<Rc<RefCell<Order>>> = vec![];

    for (idx, v) in all_nums.iter().enumerate() {
        let order: Rc<RefCell<Order>> = Rc::new(RefCell::new(Order { val: *v * mult, current_position: idx }));
        original_order.push(Rc::clone(&order));
        new_order.push(Rc::clone(&order));
    }

    let num_vals: usize = new_order.len();
    let rot_order: usize = num_vals - 1;
    let mut current_location: usize;
    let mut new_location: usize;
    let mut mv_val: i64;
    let mut order: Rc<RefCell<Order>>;

    for _ in 0..iter {
        for idx in 0..num_vals {
            current_location = original_order[idx].borrow().current_position;
            mv_val = new_order[current_location].borrow().val;
            new_location = ((current_location as i64) + mv_val).rem_euclid(rot_order as i64) as usize;

            order = new_order.remove(current_location);
            new_order.insert(new_location, order);

            for jdx in min(current_location, new_location)..max(current_location, new_location) + 1 {
                new_order[jdx].borrow_mut().current_position = jdx;
            }
        }
    }

    let zero: usize = original_order[zero_loc].borrow().current_position;

    return [1000, 2000, 3000].iter().map(|idx| new_order[(zero + idx) % num_vals].borrow().val).sum();
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input20.txt".to_string()
    ].iter().collect();

    let mut num: i64;
    let mut all_nums: Vec<i64> = vec![];
    let mut zero_loc: usize = 0;

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(num_str) = line {
                num = num_str.parse::<i64>().unwrap();
                all_nums.push(num);
                if num == 0 { zero_loc = idx; }
            }
        }
    }

    return (19, mix(&all_nums, zero_loc, 1, 1) as u64, mix(&all_nums, zero_loc, 10, 811589153) as u64)
}
