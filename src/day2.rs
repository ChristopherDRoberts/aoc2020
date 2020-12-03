use std::fs;

// TODO: Fix this mess

fn read_input(path: &str) -> (Vec<String>, Vec<String>) {
    let input: Vec<String> = fs::read_to_string(path)
        .expect("Read error")
        .lines()
        .map(|l| l.to_string())
        .collect();
    let mut policies = Vec::<String>::new();
    let mut passwords = Vec::<String>::new();
    for line in input {
        let split_line: Vec<&str> = line.split(": ").collect();
        policies.push(split_line[0].to_string());
        passwords.push(split_line[1].to_string());
    }
    return (policies, passwords);
}

fn parse_policy(policy: &String) -> (i32, i32, char) {
    let v: Vec<&str> = policy.split(|c| c == '-' || c == ' ').collect();
    let low = v[0].parse::<i32>().expect("Parse error");
    let high = v[1].parse::<i32>().expect("Parse error");
    let letter = v[2].chars().next().unwrap();
    return (low, high, letter);
}

pub fn solve(input_path: &str) {
    let (policies, passwords) = read_input(input_path);
    let mut range_count: i32 = 0;
    let mut index_count: i32 = 0;
    for i in 0..policies.len() {
        let (low, high, letter) = parse_policy(&policies[i]);
        let count = passwords[i].matches(letter).count() as i32;
        if (count >= low) && (count <= high) {
            range_count += 1;
        }
        let password = &passwords[i];
        if (password.chars().nth((low - 1) as usize).unwrap() == letter)
            ^ (password.chars().nth((high - 1) as usize).unwrap() == letter)
        {
            index_count += 1;
        }
    }
    println!("Part 1: {}\nPart 2: {}", range_count, index_count);
}
