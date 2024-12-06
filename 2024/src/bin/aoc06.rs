use std::{
    fmt::{Debug, Display, Formatter, Write},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    // Part one
    let untouched_map = parse_input(BufReader::new(f));
    {
        // part one
        let mut map = untouched_map.clone();
        // println!("{}", map);
        while map.step_guard().expect("no infinite loops") {
            // Keep doing that
            // println!("{}", map);
        }
        let p1 = map
            .map
            .iter()
            .filter(|&sq| *sq == MapSquare::Visited)
            .count();
        dbg!(p1);
    }
    // Part two
    // Brute force: try putting the obstacle on all blanks and count the infinite loops.
    // Remember to build with --release, i.e., `cargo run --release --bin aoc06 ./inputs/input06.txt`
    let mut p2 = 0;
    for (i, sq) in untouched_map.map.iter().enumerate() {
        if *sq != MapSquare::Empty {
            continue;
        }
        let mut map = untouched_map.clone();
        map.map[i] = MapSquare::Obstacle;
        loop {
            match map.step_guard() {
                Ok(true) => continue,
                Ok(false) => break,
                Err(_) => {
                    p2 += 1;
                    break;
                }
            }
        }
    }
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Map {
    let mut map = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut guard_xy = (0, 0);
    let mut guard_dxdy = (0, 0);
    for line in r.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let line_width = line.len();
        if width == 0 {
            width = line_width;
        } else {
            assert!(width == line_width, "consistent input");
        }
        for (x, ch) in line.chars().enumerate() {
            let sq = ch.into();
            // Assumption: only one guard in the input.
            if let MapSquare::Guard(dx, dy) = sq {
                guard_xy = (x, height);
                guard_dxdy = (dx, dy);
                map.push(MapSquare::Visited);
            } else {
                map.push(sq);
            }
        }
        height += 1;
    }
    let history = vec![SeenBefore::default(); map.len()];
    Map {
        map,
        history,
        width,
        height,
        guard_xy,
        guard_dxdy,
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MapSquare {
    Empty,
    Visited,
    Obstacle,
    Guard(usize, usize),
}

impl From<char> for MapSquare {
    fn from(value: char) -> Self {
        match value {
            '.' => MapSquare::Empty,
            'X' => MapSquare::Visited,
            '#' => MapSquare::Obstacle,
            '^' => MapSquare::Guard(0, usize::MAX),
            'v' => MapSquare::Guard(0, 1),
            '<' => MapSquare::Guard(usize::MAX, 0),
            '>' => MapSquare::Guard(1, 0),
            _ => unreachable!("malformed input"),
        }
    }
}

impl From<MapSquare> for char {
    fn from(value: MapSquare) -> Self {
        match value {
            MapSquare::Empty => '.',
            MapSquare::Visited => 'X',
            MapSquare::Obstacle => '#',
            MapSquare::Guard(0, usize::MAX) => '^',
            MapSquare::Guard(0, 1) => 'v',
            MapSquare::Guard(usize::MAX, 0) => '<',
            MapSquare::Guard(1, 0) => '>',
            _ => unreachable!("malformed input"),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct SeenBefore {
    /// [up, down, left, right]
    directions: [bool; 4],
}

impl SeenBefore {
    fn insert(&mut self, (x, y): (usize, usize)) -> bool {
        let idx = match (x, y) {
            (0, usize::MAX) => 0,
            (0, 1) => 1,
            (usize::MAX, 0) => 2,
            (1, 0) => 3,
            _ => unreachable!("nonsense input"),
        };
        let old = self.directions[idx];
        self.directions[idx] = true;
        old
    }
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<MapSquare>,
    history: Vec<SeenBefore>,
    width: usize,
    height: usize,
    guard_xy: (usize, usize),
    guard_dxdy: (usize, usize),
}

impl Map {
    pub fn step_guard(&mut self) -> Result<bool, &str> {
        let maybe_x = self.guard_xy.0.wrapping_add(self.guard_dxdy.0);
        let maybe_y = self.guard_xy.1.wrapping_add(self.guard_dxdy.1);
        if !(0..self.width).contains(&maybe_x) {
            return Ok(false);
        }
        if !(0..self.height).contains(&maybe_y) {
            return Ok(false);
        }
        if let MapSquare::Obstacle = self.map[maybe_y * self.width + maybe_x] {
            self.guard_dxdy = Self::turn_right(self.guard_dxdy);
            // Assumption: no infinite loops; guard is never boxed in and can always reach an exit, eventually.
            // Cheating: the input has the guard start in the open, so no four-sided boxes will happen.
            return self.step_guard();
        }
        self.guard_xy = (maybe_x, maybe_y);
        let idx = maybe_y * self.width + maybe_x;
        self.map[idx] = MapSquare::Visited;
        let been_here_before = self.history[idx].insert(self.guard_dxdy);
        if been_here_before {
            Err("infinite loop")
        } else {
            Ok(true)
        }
    }

    fn turn_right((x, y): (usize, usize)) -> (usize, usize) {
        match (x, y) {
            (0, usize::MAX) => (1, 0),
            (0, 1) => (usize::MAX, 0),
            (usize::MAX, 0) => (0, usize::MAX),
            (1, 0) => (0, 1),
            _ => unreachable!("nonsense input"),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.guard_xy;
        let (dx, dy) = self.guard_dxdy;
        let pm = |u| if u == usize::MAX { -1_i32 } else { u as i32 };
        f.write_fmt(format_args!(
            "Guard: ({x},{y})\t+/-({},{})\n",
            pm(dx),
            pm(dy)
        ))?;
        let mut i = 0;
        for _ in 0..self.height {
            for _ in 0..self.width {
                let sq = self.map[i].clone();
                i += 1;
                f.write_char(sq.into())?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}
