use std::env;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    day9::solve(&args[1]);
}