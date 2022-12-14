extern crate core;

mod utils;
mod problem20221201;
mod problem20221202;
mod problem20221203;
mod problem20221204;
mod problem20221205;
mod problem20221206;
mod problem20221207;
mod problem20221208;
mod problem20221209;
mod problem20221210;
mod problem20221211;
mod problem20221212;
mod problem20221213;
mod problem20221214;
mod problem20221215;
mod problem20221216;
mod problem20221217;
mod problem20221218;
mod problem20221219;
mod problem20221220;
mod problem20221221;
mod problem20221222;
mod problem20221223;
mod problem20221224;
mod problem20221225;

fn process_answer(answers: &mut Vec<String>, idx: usize, ans0: String, ans1: String) {
    while answers.len() <= idx {
        answers.push("".to_string());
    }
    assert_eq!(answers[idx], "");
    answers[idx] = format!("{} {}", ans0, ans1)
}

fn main() {
    let mut answers: Vec<String> = vec![];

    // u64, u64
    for daily_fn in [
        problem20221201::problem,
        problem20221202::problem,
        problem20221203::problem,
        problem20221204::problem,
        problem20221206::problem,
        problem20221207::problem,
        problem20221208::problem,
        problem20221209::problem,
        problem20221211::problem,
        problem20221212::problem,
        problem20221213::problem,
        problem20221214::problem,
        problem20221215::problem,
        problem20221216::problem,
        problem20221217::problem,
        problem20221218::problem,
        problem20221219::problem,
        problem20221220::problem,
        problem20221221::problem,
        problem20221222::problem,
        problem20221223::problem,
        problem20221224::problem,
    ] {
        let (idx, ans0, ans1) = daily_fn();
        process_answer(&mut answers, idx, format!("{}", ans0), format!("{}", ans1));
    }

    // String, String
    for daily_fn in [
        problem20221205::problem,
        problem20221225::problem,
    ] {
        let (idx, ans0, ans1) = daily_fn();
        process_answer(&mut answers, idx, ans0, ans1);
    }

    // i64, String
    for daily_fn in [
        problem20221210::problem,
    ] {
        let (idx, ans0, ans1) = daily_fn();
        process_answer(&mut answers, idx, format!("{}", ans0), format!("{}", ans1));
    }

    for (idx, ans) in answers.iter().enumerate() {
        println!("Day {} {}", idx+1, ans);
    }

}
