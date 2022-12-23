use std::cmp::{min, Ordering};
use std::path::PathBuf;

use crate::utils::read_lines;


#[derive(Debug, PartialEq)]
enum NList<T>{
    El(T),
    List(Vec<NList<T>>),
}


#[derive(Debug, Eq, PartialEq)]
enum Order {
    Right,
    Eq,
    NotRight,
}


fn nlist_from_str(str: &String) -> NList<u64> {
    return NList::List(
        get_elements(&open_list(&str)).iter().map(|el|
            if is_int(&el) {
                NList::El(el.parse::<u64>().unwrap())
            } else {
                nlist_from_str(&el)
            }).collect()
    )
}


// remove brackets
fn open_list(str: &String) -> String {
    if str.len() == 0 { return str.to_owned() }
    return if str[0..1] != "[".to_string() { str.to_owned() } else { str[1..str.len() - 1].to_owned() }
}


// elements of a csv
fn get_elements(str: &String) -> Vec<String> {
    let mut num_brackets: u64 = 0;
    let mut ret: Vec<String> = vec![];
    let mut buf: String = "".to_string();
    for c in str.chars() {
        if c == '[' { num_brackets += 1; } else if c == ']' { num_brackets -= 1; }
        if num_brackets == 0 && c == ',' {
            ret.push(buf);
            buf = "".to_string();
        } else {
            buf.push(c);
        }
    }
    if buf.len() > 0 { ret.push(buf); }
    return ret
}


fn is_int(str: &String) -> bool {
    return if str[0..1] == "[".to_string() { false } else { true }
}


fn compare(left_str: &String, right_str: &String) -> Order {
    return if is_int(left_str) && is_int(right_str) {
        let left_val: u64 = left_str.parse::<u64>().unwrap();
        let right_val: u64 = right_str.parse::<u64>().unwrap();
        if left_val < right_val {
            Order::Right
        } else if left_val > right_val {
            Order::NotRight
        } else {
            Order::Eq
        }
    } else if is_int(left_str) {
        compare(&format!("[{}]", left_str), right_str)
    } else if is_int(right_str) {
        compare(left_str, &format!("[{}]", right_str))
    } else {
        let left_el: Vec<String> = get_elements(&open_list(left_str));
        let right_el: Vec<String> = get_elements(&open_list(right_str));

        let left_num = left_el.len();
        let right_num = right_el.len();
        let mut comp: Order;
        for idx in 0..min(left_num, right_num) {
            comp = compare(&left_el[idx], &right_el[idx]);
            if comp != Order::Eq { return comp }
        }
        if left_num == right_num { Order::Eq } else if left_num < right_num { Order::Right } else { Order::NotRight }
    }
}


fn compare_nlist(left: &NList<u64>, right: &NList<u64>) -> Order {
    match (left, right) {
        (NList::El(left_val), NList::El(right_val)) => {
            if left_val < right_val { Order::Right }
            else if right_val < left_val { Order::NotRight }
            else { Order::Eq }
        }
        (NList::El(l), list) => { compare_nlist(&NList::List(vec![NList::El(*l)]), list) },
        (list, NList::El(l)) => { compare_nlist(list, &NList::List(vec![NList::El(*l)])) },
        (NList::List(left_el), NList::List(right_el)) => {

            let left_num = left_el.len();
            let right_num = right_el.len();
            let mut comp: Order;
            for idx in 0..min(left_num, right_num) {
                comp = compare_nlist(&left_el[idx], &right_el[idx]);
                if comp != Order::Eq { return comp }
            }
            if left_num == right_num { Order::Eq } else if left_num < right_num { Order::Right } else { Order::NotRight }
        }
    }
}


fn compare_nlist_ord(left: &NList<u64>, right: &NList<u64>) -> Ordering {
    match compare_nlist(left, right) {
        Order::Right => Ordering::Less,
        Order::NotRight => Ordering::Greater,
        Order::Eq => Ordering::Equal,
    }
}


pub fn problem() -> (usize, u64, u64) {
    let data_dir: String = env!("CARGO_MANIFEST_DIR").to_owned();
    let data_path: PathBuf = [
        data_dir,
        "src".to_string(),
        "input13.txt".to_string()
    ].iter().collect();

    let mut left_str: String = "".to_string();
    let mut right_str: String = "".to_string();
    let mut result: u64 = 0;
    let mut all_packets: Vec<String> = vec!["[[2]]".to_string(), "[[6]]".to_string()];

    if let Ok(lines) = read_lines(data_path) {
        for (idx, line) in lines.enumerate() {
            if let Ok(str_var) = line {

                let str_var_copy = str_var.to_owned();

                if idx % 3 == 0 { left_str = str_var; all_packets.push(str_var_copy); }
                else if idx % 3 == 1 { right_str = str_var; all_packets.push(str_var_copy); }
                else { match compare(&left_str, &right_str) {
                        Order::Right => { result += 1 + (idx as u64 / 3); },
                        _ => {},
                } }
            }
        }
    }

    let mut all_packets_nlist: Vec<NList<u64>> = all_packets.iter().map(nlist_from_str).collect();
    all_packets_nlist.sort_by(compare_nlist_ord);

    let dpac0: NList<u64> = nlist_from_str(&"[[2]]".to_string());
    let dpac1: NList<u64> = nlist_from_str(&"[[6]]".to_string());

    let decoder_key: u64 = all_packets_nlist.iter().enumerate()
        .filter(
            |(_, p)| **p == dpac0 || **p == dpac1
        )
        .map(|(idx, _)| (1+idx) as u64)
        .product();

    return (12, result, decoder_key)
}
