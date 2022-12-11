use std::path::PathBuf;

use crate::utils::read_lines;


struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
}


fn get_monkeys(real: bool) -> Vec<Monkey> {
    let monkeys_real: Vec<Monkey> = vec![
        Monkey {
            items: vec![57],
            operation: Box::new(|x| x * 13),
            test: Box::new(|x| if x % 11 == 0 { 3 } else { 2 })
        },
        Monkey {
            items: vec![58, 93, 88, 81, 72, 73, 65],
            operation: Box::new(|x| x + 2),
            test: Box::new(|x| if x % 7 == 0 { 6 } else { 7 })
        },
        Monkey {
            items: vec![65, 95],
            operation: Box::new(|x| x + 6),
            test: Box::new(|x| if x % 13 == 0 { 3 } else { 5 })
        },
        Monkey {
            items: vec![58, 80, 81, 83],
            operation: Box::new(|x| x * x),
            test: Box::new(|x| if x % 5 == 0 { 4 } else { 5 })
        },
        Monkey {
            items: vec![58, 89, 90, 96, 55],
            operation: Box::new(|x| x + 3),
            test: Box::new(|x| if x % 3 == 0 { 1 } else { 7 })
        },
        Monkey {
            items: vec![66, 73, 87, 58, 62, 67],
            operation: Box::new(|x| x * 7),
            test: Box::new(|x| if x % 17 == 0 { 4 } else { 1 })
        },
        Monkey {
            items: vec![85, 55, 89],
            operation: Box::new(|x| x + 4),
            test: Box::new(|x| if x % 2 == 0 { 2 } else { 0 })
        },
        Monkey {
            items: vec![73, 80, 54, 94, 90, 52, 69, 58],
            operation: Box::new(|x| x + 7),
            test: Box::new(|x| if x % 19 == 0 { 6 } else { 0 })
        },
    ];

       let monkeys_test: Vec<Monkey> = vec![
        Monkey {
            items: vec![79, 98],
            operation: Box::new(|x| x * 19),
            test: Box::new(|x| if x % 23 == 0 { 2 } else { 3 })
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: Box::new(|x| x + 6),
            test: Box::new(|x| if x % 19 == 0 { 2 } else { 0 })
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: Box::new(|x| x * x),
            test: Box::new(|x| if x % 13 == 0 { 1 } else { 3 })
        },
        Monkey {
            items: vec![74],
            operation: Box::new(|x| x + 3),
            test: Box::new(|x| if x % 17 == 0 { 0 } else { 1 })
        },
    ];

    return if real { monkeys_real } else { monkeys_test }
}


fn run_monkeys0(monkeys: Vec<Monkey>) -> u64 {
    let mut monkeys = monkeys;

    let num_monkeys: usize = monkeys.len();
    let mut monkey_action: Vec<usize> = vec![];
    for _ in 0..monkeys.len() { monkey_action.push(0) }

    let mut updates: Vec<(usize, u64)> = vec![];
    for _ in 0..20 {
        for idx in 0..num_monkeys {

            let monkey: &mut Monkey = &mut monkeys[idx];

            monkey_action[idx] += monkey.items.len();

            for item in monkey.items.iter() {
                let new_value: u64 = (monkey.operation)(*item) / 3;
                let new_idx: usize = (monkey.test)(new_value);
                updates.push((new_idx, new_value));
            }
            monkey.items = vec![];

            for (new_idx, value) in &updates {
                monkeys[*new_idx].items.push(*value);
            }
            updates = vec![];
        }
    }

    monkey_action.sort();
    monkey_action.reverse();

    (monkey_action[0] * monkey_action[1]) as u64
}


fn strip_and_parse(num: &str) -> u64 {
    num.chars().filter(|x| "0123456789".contains(*x)).collect::<String>().parse::<u64>().unwrap()
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input11.txt".to_string()
    ].iter().collect();

    let mut init_vals: Vec<Vec<u64>> = vec![];
    let mut mods: Vec<u64> = vec![];
    let mut throw_true: Vec<usize> = vec![];
    let mut throw_false: Vec<usize> = vec![];

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(line) = line {
                if line.contains("divisible by ") {
                    mods.push(line.split("divisible by ").collect::<Vec<&str>>()[1].parse().unwrap());
                }
                if line.contains("If true: throw to monkey ") {
                    throw_true.push(line.split("If true: throw to monkey ").collect::<Vec<&str>>()[1].parse().unwrap());
                }
                if line.contains("If false: throw to monkey ") {
                    throw_false.push(line.split("If false: throw to monkey ").collect::<Vec<&str>>()[1].parse().unwrap());
                }
                if line.contains("Starting items: ") {
                    let csv: Vec<u64> = line.split("Starting items: ").collect::<Vec<&str>>()[1].split(&",").map(|x|strip_and_parse(x)).collect();
                    init_vals.push(csv);
                }
            }
        }
    }
    let mut monkey_action: Vec<usize> = vec![];
    for _ in 0..mods.len() { monkey_action.push(0) }

    let mut monkey_vals: Vec<Vec<Vec<u64>>> = init_vals.iter().map(|x| x.iter().map(|y| mods.iter().map(|m| y % m).collect()).collect()).collect();

    let ops = [|x| x * 13, |x| x + 2, |x| x + 6, |x| x * x, |x| x + 3, |x| x * 7, |x| x + 4, |x| x + 7];

    let mut next_idx: usize;
    let mut updates: Vec<(usize, Vec<u64>)> = vec![];
    for _ in 0..10000 {
        for idx in 0..mods.len() {
            monkey_action[idx] += &monkey_vals[idx].len();
            let vals: &mut Vec<Vec<u64>> = &mut monkey_vals[idx];
            while let Some(mut val) = vals.pop() {
                val = val.iter().zip(&mods).map(|(&x, &y)| (ops[idx])(x) % y).collect();
                next_idx = if val[idx] == 0 { throw_true[idx] } else { throw_false[idx] };
                updates.push((next_idx, val));
            }
            while let Some((next_idx, val)) = updates.pop() { monkey_vals[next_idx].push(val); }
        }
    }
    monkey_action.sort();
    monkey_action.reverse();

    return (10, run_monkeys0(get_monkeys(true)), (monkey_action[0] * monkey_action[1]) as u64)
}
