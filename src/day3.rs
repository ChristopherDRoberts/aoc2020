use std::fs;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: Vec<Vec<bool>>,
}

impl Map {
    fn new(path: &str) -> Self {
        let input = fs::read_to_string(path).expect("Read error");
        let mut v = Vec::<Vec<bool>>::new();
        for line in input.lines() {
            v.push(
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => false,
                    })
                    .collect(),
            )
        }
        Map {
            height: v.len(),
            width: v[0].len(),
            trees: v,
        }
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn collision(&self, x: usize, y: usize) -> bool {
        return self.trees[y][x % self.width()];
    }
    
    fn collisions_along_path(&self, dx: usize, dy: usize) -> usize {
        let mut collisions: usize = 0;
        for y in (0..self.height()).step_by(dy) {
            if self.collision((dx * y) / dy, y) {
                collisions += 1;
            }
        }
        collisions
    }
}

pub fn solve(path: &str) {
    let map = Map::new(path);
    let part1 = map.collisions_along_path(3, 1);
    let mut part2 = 1;
    for (dx, dy) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let tmp = map.collisions_along_path(dx, dy);
        part2 *= tmp;
    }
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
