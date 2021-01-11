use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> Vec<i64> {
    let mut input: Vec<i64> = fs::read_to_string(path)
        .expect("Read error")
        .lines()
        .map(|l| l.parse::<i64>().expect("Parse error"))
        .collect();
    input.sort();
    input.insert(0, 0);
    input.push(input[input.len() - 1] + 3);
    return input;
}

fn paths_to(adaptor: i64, adaptors: &HashSet<i64>, cache: &mut HashMap<i64, i64>) -> i64 {
    if adaptor == 0 {
        return 1;
    } else if cache.contains_key(&adaptor) {
        return cache[&adaptor];
    } else {
        let mut count = 0;
        for i in 1..=3 {
            println!("{}", &(adaptor - i));
            if adaptors.contains(&(adaptor - i)) {
                let res = paths_to(adaptor - i, adaptors, cache);
                cache.insert(adaptor - i, res);
                count += res;
            }
        }
        return count;
    }
}

pub fn solve(path: &str) {
    let input = read_input(path);
    let diff = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    println!("{:?}\n{:?}", &input, &diff);
    let part1 = diff.iter().filter(|&x| *x == 1).count() * diff.iter().filter(|&x| *x == 3).count();

    let adaptors: HashSet<i64> = input.iter().cloned().collect();
    let mut cache = HashMap::<i64, i64>::new();
    let part2 = paths_to(input[input.len() - 1], &adaptors, &mut cache);
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
