use std::f64::consts::PI;
use std::fs;

// over-engineered solution with support for arbitrary headings and movement distances

#[derive(Debug)]
struct Ship {
    x: f64,
    y: f64,
    bearing: f64,
}

impl Ship {
    fn new(x: f64, y: f64, bearing: f64) -> Self {
        Ship {
            x: x,
            y: y,
            bearing: bearing, // East is zero bearing, not North
        }
    }

    fn turn(&mut self, rad: f64) {
        self.bearing = (self.bearing + rad) % (2.0 * PI);
    }

    // 'move' is reserved
    fn mov(&mut self, dist: f64, bearing: f64) {
        self.x = self.x + dist * bearing.cos();
        self.y = self.y + dist * bearing.sin();
    }

    fn mov_to_waypoint(&mut self, dist: f64, waypoint: &Waypoint) {
        self.x = self.x + dist * waypoint.x;
        self.y = self.y + dist * waypoint.y;
    }

    fn get_bearing(&self) -> f64 {
        self.bearing
    }

    fn manhattan_dist(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Waypoint {
    x: f64,
    y: f64,
}

impl Waypoint {
    fn new(x: f64, y: f64) -> Self {
        Waypoint { x: x, y: y }
    }

    fn rotate(&mut self, rad: f64) {
        let x = self.x;
        let y = self.y;
        self.x = x * rad.cos() - y * rad.sin();
        self.y = x * rad.sin() + y * rad.cos();
    }

    fn mov(&mut self, dist: f64, bearing: f64) {
        self.x = self.x + dist * bearing.cos();
        self.y = self.y + dist * bearing.sin();
    }
}

fn read_input(path: &str) -> Vec<(char, f64)> {
    let mut instructions = Vec::<(char, f64)>::new();
    let input = fs::read_to_string(path).expect("Read error");
    for ins in input.lines() {
        let c = ins.chars().next().expect("Parse error");
        let x = &ins[1..].parse::<f64>().expect("Parse error");
        instructions.push((c, *x));
    }
    return instructions;
}

fn part1(instructions: &Vec<(char, f64)>) -> f64 {
    let mut ship = Ship::new(0.0, 0.0, 0.0);
    for ins in instructions {
        match ins.0 {
            'E' => ship.mov(ins.1, 0.0),
            'N' => ship.mov(ins.1, PI / 2.0),
            'W' => ship.mov(ins.1, PI),
            'S' => ship.mov(ins.1, 3.0 * PI / 2.0),
            'L' => ship.turn((ins.1).to_radians()),
            'R' => ship.turn((-ins.1).to_radians()),
            'F' => ship.mov(ins.1, ship.get_bearing()),
            _ => (),
        }
    }
    ship.manhattan_dist()
}

fn part2(instructions: &Vec<(char, f64)>) -> f64 {
    let mut ship = Ship::new(0.0, 0.0, 0.0);
    let mut waypoint = Waypoint::new(10.0, 1.0);
    for ins in instructions {
        match ins.0 {
            'E' => waypoint.mov(ins.1, 0.0),
            'N' => waypoint.mov(ins.1, PI / 2.0),
            'W' => waypoint.mov(ins.1, PI),
            'S' => waypoint.mov(ins.1, 3.0 * PI / 2.0),
            'L' => waypoint.rotate((ins.1).to_radians()),
            'R' => waypoint.rotate((-ins.1).to_radians()),
            'F' => ship.mov_to_waypoint(ins.1, &waypoint),
            _ => (),
        }
    }
    ship.manhattan_dist()
}

pub fn solve(path: &str) {
    let instructions = read_input(path);
    print!(
        "Part 1: {}\nPart 2: {}",
        part1(&instructions),
        part2(&instructions)
    );
}