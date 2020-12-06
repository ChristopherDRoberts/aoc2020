use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn read_input(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .expect("Read error")
        .split("\n\n")
        .map(|l| {
            l.replace("\n", " ")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_passports(input: Vec<Vec<String>>) -> Vec<HashMap<String, String>> {
    let mut passports = Vec::new();
    for passport in input {
        let mut map = HashMap::<String, String>::new();
        for field_pair in passport {
            let key_val = field_pair
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            map.insert(key_val[0].clone(), key_val[1].clone());
        }
        passports.push(map);
    }
    return passports;
}

fn has_required_fields(passport: &HashMap<String, String>, valid_keys: &Vec<String>) -> bool {
    valid_keys.iter().all(|k| passport.contains_key(k))
}

fn validate_passport(
    passport: &HashMap<String, String>,
    validators: &HashMap<String, Regex>,
) -> bool {
    let mut is_valid = true;

    for (key, val) in passport.iter() {
        if key == "cid" {
            continue;
        }
        if !validators.get(key).expect("Missing key").is_match(val) {
            is_valid = false;
            break;
        }
    }

    return is_valid;
}

pub fn solve(path: &str) {
    // setup
    let valid_keys: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    ];

    let mut validators = HashMap::<String, Regex>::new();
    validators.insert(
        "byr".to_string(),
        Regex::new(r"19[2-9][0-9]|200[0-2]").expect("Regex compile error"),
    );
    validators.insert(
        "iyr".to_string(),
        Regex::new(r"201[0-9]|2020").expect("Regex compile error"),
    );
    validators.insert(
        "eyr".to_string(),
        Regex::new(r"202[0-9]|2030").expect("Regex compile error"),
    );
    validators.insert(
        "hgt".to_string(),
        Regex::new(r"(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in").expect("Regex compile error"),
    );
    validators.insert(
        "hcl".to_string(),
        Regex::new(r"#[0-9a-f]{6}").expect("Regex compile error"),
    );
    validators.insert(
        "ecl".to_string(),
        Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").expect("Regex compile error"),
    );
    validators.insert(
        "pid".to_string(),
        Regex::new(r"^\d{9}$").expect("Regex compile error"),
    );

    let input = read_input(path);
    let passports = get_passports(input);
    let mut part1 = 0;
    let mut part2 = 0;
    for passport in passports {
        if has_required_fields(&passport, &valid_keys) {
            part1 += 1;
            if validate_passport(&passport, &validators) {
                part2 += 1;
            }
        }
    }
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
