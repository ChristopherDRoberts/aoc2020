use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .expect("Read Error")
        .split("\n\n")
        .map(|l| l.split('\n').map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

pub fn solve(path: &str) {
    let input = read_input(path);

    let mut part1 = 0;
    for group in &input {
        let mut set = HashSet::<char>::new();
        for respondent in group {
            for response in respondent.chars() {
                set.insert(response);
            }
        }
        part1 += set.len();
    }

    let mut part2 = 0;
    for group in &input {
        let mut common_answers: HashSet<char> = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .iter()
        .cloned()
        .collect();
        for respondent in group {
            let mut set = HashSet::<char>::new();
            for response in respondent.chars() {
                set.insert(response);
            }
            common_answers = common_answers.intersection(&set).cloned().collect();
        }
        part2 += common_answers.len();
    }

    println!("Part1: {}\nPart 2: {}", part1, part2);
}
