use std::fs;

fn read_input(path: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::<Vec<char>>::new();
    let input = fs::read_to_string(path).expect("Read Error");
    for row in input.lines() {
        let mut grid_row = Vec::new();
        for col in row.chars() {
            grid_row.push(col);
        }
        grid.push(grid_row);
    }
    return grid;
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    let mut prev_grid = grid.clone();
    let mut next_grid = evolve_grid(&prev_grid, empty_rule, occupied_rule);
    while !stabilised(&prev_grid, &next_grid) {
        prev_grid = next_grid;
        next_grid = evolve_grid(&prev_grid, empty_rule, occupied_rule);
    }
    next_grid
        .into_iter()
        .flatten()
        .filter(|pos| *pos == '#')
        .count()
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut prev_grid = grid.clone();
    let mut next_grid = evolve_grid(
        &prev_grid,
        empty_rule_line_of_sight,
        occupied_rule_line_of_sight,
    );
    while !stabilised(&prev_grid, &next_grid) {
        prev_grid = next_grid;
        next_grid = evolve_grid(
            &prev_grid,
            empty_rule_line_of_sight,
            occupied_rule_line_of_sight,
        );
    }
    next_grid
        .into_iter()
        .flatten()
        .filter(|pos| *pos == '#')
        .count()
}

fn evolve_grid(
    prev_grid: &Vec<Vec<char>>,
    empty_rule: fn(isize, isize, &Vec<Vec<char>>) -> bool,
    occupied_rule: fn(isize, isize, &Vec<Vec<char>>) -> bool,
) -> Vec<Vec<char>> {
    let mut next_grid = prev_grid.clone();
    for row in 0..prev_grid.len() {
        for col in 0..prev_grid[0].len() {
            if prev_grid[row][col] == 'L' && empty_rule(row as isize, col as isize, &prev_grid) {
                next_grid[row][col] = '#';
            } else if prev_grid[row][col] == '#'
                && occupied_rule(row as isize, col as isize, &prev_grid)
            {
                next_grid[row][col] = 'L'
            }
        }
    }
    return next_grid;
}

fn empty_rule(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
    let max_row = (grid.len() - 1) as isize;
    let max_col = (grid[0].len() - 1) as isize;
    for i in row - 1..=row + 1 {
        if i < 0 || i > max_row {
            continue;
        }
        for j in col - 1..=col + 1 {
            if j < 0 || j > max_col {
                continue;
            }
            if i == row && j == col {
                continue;
            }
            if grid[i as usize][j as usize] == '#' {
                return false;
            }
        }
    }
    return true;
}

fn occupied_rule(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
    let max_row = (grid.len() - 1) as isize;
    let max_col = (grid[0].len() - 1) as isize;
    let mut count = 0;
    for i in row - 1..=row + 1 {
        if i < 0 || i > max_row {
            continue;
        }
        for j in col - 1..=col + 1 {
            if count >= 4 {
                return true;
            }
            if j < 0 || j > max_col {
                continue;
            }
            if i == row && j == col {
                continue;
            }
            if grid[i as usize][j as usize] == '#' {
                count += 1;
            }
        }
    }
    return count >= 4;
}

fn empty_rule_line_of_sight(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
    for d_row in -1..=1 {
        for d_col in -1..=1 {
            if d_row == 0 && d_col == 0 {
                continue;
            }
            if line_of_sight(row, col, &grid, d_row, d_col) {
                return false;
            }
        }
    }
    return true;
}

fn occupied_rule_line_of_sight(row: isize, col: isize, grid: &Vec<Vec<char>>) -> bool {
    let mut count = 0;
    for d_row in -1..=1 {
        for d_col in -1..=1 {
            if count >= 5 {
                return true;
            }
            if d_row == 0 && d_col == 0 {
                continue;
            }
            if line_of_sight(row, col, &grid, d_row, d_col) {
                count += 1;
            }
        }
    }
    return count >= 5;
}

fn line_of_sight(
    row: isize,
    col: isize,
    grid: &Vec<Vec<char>>,
    d_row: isize,
    d_col: isize,
) -> bool {
    let max_row = (grid.len() - 1) as isize;
    let max_col = (grid[0].len() - 1) as isize;
    let mut i = row + d_row;
    let mut j = col + d_col;
    while i >= 0 && i <= max_row && j >= 0 && j <= max_col {
        match grid[i as usize][j as usize] {
            '#' => return true,
            'L' => return false,
            _ => (),
        }
        i += d_row;
        j += d_col;
    }
    return false;
}

fn stabilised(prev: &Vec<Vec<char>>, next: &Vec<Vec<char>>) -> bool {
    prev == next
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    println!();
    for row in grid {
        println!();
        for position in row {
            print!("{}", position);
        }
    }
}

pub fn solve(path: &str) {
    let grid = read_input(path);
    println!("Part 1: {}\nPart 2: {}", part1(&grid), part2(&grid));
}
