use std::env;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    day3::solve(&args[1]);
}
