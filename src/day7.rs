use std::collections::HashMap;
use std::fs;

// There has to be a better way to do this
fn read_input(path: &str) -> HashMap<String, Vec<(u32, String)>> {
    let mut rules = HashMap::<String, Vec<(u32, String)>>::new();
    let input = fs::read_to_string(path).expect("Read error");
    for line in input.lines() {
        let proc_line = line
            .replace(" contain ", ":")
            .replace(" bags", "")
            .replace(" bag", "")
            .replace(".", "")
            .replace(", ", ",");
        let split_line = proc_line
            .split(|c| c == ':' || c == ',')
            .collect::<Vec<_>>();
        let key = split_line[0].to_string();
        let mut rule = Vec::<(u32, String)>::new();
        for i in 1..split_line.len() {
            if split_line[i] == "no other" {
                rule.push((0, String::from("no other")));
            } else {
                let count = split_line[i].chars().next().unwrap().to_digit(10).unwrap();
                rule.push((count, split_line[i][2..].to_string()));
            }
        }
        rules.insert(key, rule);
    }
    return rules;
}

fn contains(bag: &str, outer_bag: &str, rules: &HashMap<String, Vec<(u32, String)>>) -> bool {
    if rules[outer_bag].len() == 1 && rules[outer_bag][0].0 == 0 {
        return false;
    }
    for rule in &rules[outer_bag] {
        if rule.1 == bag {
            return true;
        }
    }
    for rule in &rules[outer_bag] {
        if contains(bag, &rule.1, rules) {
            return true;
        }
    }
    return false;
}

fn counts(outer_bag: &str, rules: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    if rules[outer_bag].len() == 1 && rules[outer_bag][0].0 == 0 {
        return 1;
    } else {
        let mut count = 0;
        for rule in &rules[outer_bag] {
            count += counts(&rule.1, rules) * rule.0;
        }
        return count + 1;
    }
}

pub fn solve(path: &str) {
    let rules = read_input(path);
    let bag = "shiny gold";
    let mut contained_count = 0;
    for outer_bag in rules.keys() {
        if contains(&bag, &outer_bag, &rules) {
            contained_count += 1;
        }
    }
    let contains_count = counts(bag, &rules) - 1; // since counts includes the outermost bag in its count
    println!("Part 1: {}\nPart 2: {}", contained_count, contains_count);
}
