mod utils;
mod problem20221201;
mod problem20221202;
mod problem20221203;
mod problem20221204;

fn main() {
    for daily_fn in [
        problem20221201::problem,
        problem20221202::problem,
        problem20221203::problem,
        problem20221204::problem,
    ] {
        println!("{:?}", daily_fn());
    }
}
