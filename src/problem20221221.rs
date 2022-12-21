use std::collections::HashMap;
use std::path::PathBuf;

use crate::utils::read_lines;


#[derive(Clone)]
struct Expr {
    v0: String,
    v1: String,
    op: char,
}


fn solve(var: &String,
         undetermined: &HashMap<String, Expr>,
         determined: &mut HashMap<String, i64>) -> i64 {
    if !determined.contains_key(var) {
        let expr = &undetermined.get(var).unwrap();
        let v0: &String = &expr.v0;
        let v1: &String = &expr.v1;

        let i0: i64 = solve(v0, undetermined, determined);
        let i1: i64 = solve(v1, undetermined, determined);

        let val: i64 = match expr.op {
            '*' => { i0 * i1 },
            '-' => { i0 - i1 },
            '+' => { i0 + i1 },
            '/' => { i0 / i1 },
            _ => panic!(),
        };
        determined.insert(var.to_string(), val);
    }

    return *determined.get(var).unwrap()
}


fn solve_sym(var: &String,
             undetermined: &HashMap<String, Expr>,
             determined: &mut HashMap<String, String>) -> String {
    if !determined.contains_key(var) {
        let expr = &undetermined.get(var).unwrap();

        let v0: String = solve_sym(&expr.v0, undetermined, determined);
        let v1: String = solve_sym(&expr.v1, undetermined, determined);

        let val: String;
        if v0.parse::<i64>().is_ok() && v1.parse::<i64>().is_ok() {
            let val0: i64 = v0.parse::<i64>().unwrap();
            let val1: i64 = v1.parse::<i64>().unwrap();

            val = match expr.op {
                '*' => { format!("{}", val0 * val1) },
                '-' => { format!("{}", val0 - val1) },
                '+' => { format!("{}", val0 + val1) },
                '/' => { format!("{}", val0 / val1) },
                _ => panic!(),
            };
        } else {
            val = match expr.op {
                '*' => { format!("({}*{})", v0, v1) },
                '-' => { format!("({}-{})", v0, v1) },
                '+' => { format!("({}+{})", v0, v1) },
                '/' => { format!("({}/{})", v0, v1) },
                _ => panic!(),
            };
        }
        determined.insert(var.to_string(), val);
    }

    return determined.get(var).unwrap().clone()
}


fn unwrap_once(expr: &String) -> (String, char, String) {
    let expr = &expr[1..expr.len()-1];

    let mut jdx: usize = 0;
    let mut parens: u64 = 0;

    for (idx, c) in expr.chars().enumerate() {
        if c == '(' { parens += 1; }
        if c == ')' { parens -= 1; }
        if (c == '*' || c == '-' || c == '/' || c == '+') && parens == 0 {
            jdx = idx;
            break
        }
    }
    return (expr[0..jdx].to_string(), expr.chars().nth(jdx).unwrap(), expr[jdx+1..].to_string())
}


fn unwrap(val: i64, expr: &String) -> i64 {

    let mut val = val;
    let mut expr = expr;

    let mut v0: String;
    let mut v1: String;
    let mut op: char;
    let mut term: i64;
    let mut first: bool;

    while expr.contains('(') {
        (v0, op, v1) = unwrap_once(expr);

        match (v0.parse::<i64>(), v1.parse::<i64>()) {
            (Ok(num), Err(_err)) => {
                first = true;
                term = num;
                expr = &v1;
            },
            (Err(_err), Ok(num)) => {
                first = false;
                term = num;
                expr = &v0;
            },
            _ => panic!()
        }

        val = match op {
            '*' => { val / term },
            '+' => { val - term },
            '-' => { if first { term - val } else { val + term } },
            '/' => { if first { term / val } else { val * term }},
            _ => panic!(),
        }
    }

    return val
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input21.txt".to_string()
    ].iter().collect();

    let mut undetermined: HashMap<String, Expr> = HashMap::new();
    let mut determined: HashMap<String, i64> = HashMap::new();
    let undetermined_flex: HashMap<String, Expr>;
    let mut determined_sym: HashMap<String, String> = HashMap::new();

    let mut key: String;
    let mut expr: String;

    if let Ok(lines) = read_lines(data_path) {
        for line in lines {
            if let Ok(expr_str) = line {

                let key_expr: Vec<&str> = expr_str.split(": ").collect();
                key = key_expr[0].to_string();
                expr = key_expr[1].to_string();

                match expr.parse::<i64>() {
                    Ok(val) => {
                        determined.insert(key.to_string(), val);
                        determined_sym.insert(key.to_string(), expr);
                    },
                    Err(_e) => {
                        for op in ['*', '/', '-', '+'] {
                            if expr.contains(op) {
                                let expr_copy: String = expr.clone();
                                let vals: Vec<&str> = expr_copy.split(&format!(" {} ", op)).collect();
                                undetermined.insert(key.to_string(), Expr { v0: vals[0].to_string(), v1: vals[1].to_string(), op });
                            }
                        }
                    },
                }

            }
        }
    }

    undetermined_flex = undetermined.clone();

    let result0: i64 = solve(&"root".to_string(), &undetermined, &mut determined);

    determined_sym.insert("humn".to_string(), "x".to_string());
    let eq0: &String = &undetermined_flex.get("root").unwrap().v0;
    let eq1: &String = &undetermined_flex.get("root").unwrap().v1;

    let arith_expr0: String = solve_sym(eq0, &undetermined_flex, &mut determined_sym);
    let arith_expr1: String = solve_sym(eq1, &undetermined_flex, &mut determined_sym);

    let result1: i64 = if arith_expr0.parse::<i64>().is_ok() {
        unwrap(arith_expr0.parse::<i64>().unwrap(), &arith_expr1)
    } else {
        unwrap(arith_expr1.parse::<i64>().unwrap(), &arith_expr0)
    };

    return (20, result0 as u64, result1 as u64)
}
