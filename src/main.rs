use std::env;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    day5::solve(&args[1]);
}
