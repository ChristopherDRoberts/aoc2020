use std::fs;

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Error reading input")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn solve(input_path: &str) {
    let expenses = read_input(input_path);
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    // shameful bruteforce
    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            if (expenses[i] + expenses[j]) == 2020 {
                part1 = expenses[i] * expenses[j];
            }
            for k in (i + 2)..expenses.len() {
                if (expenses[i] + expenses[j] + expenses[k]) == 2020 {
                    part2 = expenses[i] * expenses[j] * expenses[k];
                }
            }
        }
    }

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}