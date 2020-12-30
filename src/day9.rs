use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .expect("Read error")
        .lines()
        .map(|l| l.parse::<i64>().expect("Parse error"))
        .collect::<Vec<_>>()
}

fn part1(numbers: &Vec<i64>, preamble_length: usize) -> (Option<i64>, usize) {
    let mut preceding: HashSet<i64> = numbers[0..preamble_length].iter().cloned().collect();
    for i in preamble_length..numbers.len() {
        if !is_valid(numbers[i], &preceding) {
            return (Some(numbers[i]), i);
        } else {
            preceding.remove(&numbers[i - preamble_length]);
            preceding.insert(numbers[i]);
        }
    }
    return (None, 0);
}

fn part2(numbers: &Vec<i64>, invalid_idx: usize) -> (i64, i64) {
    for min_idx in 0..invalid_idx {
        let mut sum = numbers[min_idx];
        for max_idx in min_idx + 1..invalid_idx {
            sum += numbers[max_idx];
            if sum == numbers[invalid_idx] {
                return (numbers[min_idx], numbers[max_idx]);
            }
        }
    }
    return (0, 0);
}

fn is_valid(next: i64, preceding: &HashSet<i64>) -> bool {
    for number in preceding {
        if preceding.contains(&(next - number)) {
            return true;
        }
    }
    return false;
}

pub fn solve(path: &str) {
    let input = read_input(path);
    let (invalid, invalid_idx) = part1(&input, 25);
    let (min, max) = part2(&input, invalid_idx);
    println!("Part 1: {}\nPart 2: {}", invalid.unwrap(), min + max);
}
