use std::fs;

fn read_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Read error")
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn get_idx(bsp: &[char], low: i32, high: i32, left: char, right: char) -> i32 {
    let mut low = low;
    let mut high = high;
    for c in bsp {
        let width = high - low;
        if *c == left {
            high = low + width / 2;
        } else if *c == right {
            low = high - width / 2;
        }
    }
    return high;
}

fn get_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

pub fn solve(path: &str) {
    let input = read_input(path);
    let mut max_id = 0;
    let mut seat_ids: Vec<i32> = Vec::new();
    for bsp in input {
        let row = get_idx(&bsp[0..7], 0, 127, 'F', 'B');
        let col = get_idx(&bsp[7..], 0, 7, 'L', 'R');
        let id = get_id(row, col);
        seat_ids.push(id);
        if id > max_id {
            max_id = id;
        }
    }
    seat_ids.sort();
    let mut missing_id = 0;
    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i + 1] - seat_ids[i] == 2 {
            missing_id = seat_ids[i] + 1;
            break;
        }
    }
    println!("Part 1: {}\nPart 2: {}", max_id, missing_id);
}
