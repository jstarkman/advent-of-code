use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let coords = parse_input(BufReader::new(f));
    let (height, width, steps) = if path_input.contains("mini") {
        (7, 7, 12)
    } else {
        (71, 71, 1024)
    };
    // println!("{coords:?}");
    let mut world = vec![vec![false; width]; height];
    simulate(&mut world, &coords[0..steps]);
    // println!("{world:?}");
    let p1 = find_length_shortest_path(&world);
    dbg!(p1);
    // Part two
    for another_step in steps..coords.len() {
        simulate(&mut world, &coords[another_step..another_step + 1]);
        let has_path = find_length_shortest_path(&world);
        if has_path.is_none() {
            println!("{},{}", coords[another_step].0, coords[another_step].1);
            break;
        }
    }
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    for line in r.lines().flat_map(|l| l.ok().into_iter()) {
        let (xs, ys) = line.split_once(',').expect("well-formed input");
        let x: usize = xs.parse().unwrap();
        let y: usize = ys.parse().unwrap();
        coords.push((x, y));
    }
    coords
}

fn simulate(world: &mut Vec<Vec<bool>>, coords: &[(usize, usize)]) {
    for &(x, y) in coords.iter() {
        world[y][x] = true;
    }
}

fn find_length_shortest_path(world: &Vec<Vec<bool>>) -> Option<u32> {
    let height = world.len();
    let width = world[0].len();
    let goal = (width - 1, height - 1);
    let mut seen = world.clone(); // have already seen the obstacles
    let mut active = vec![];
    active.push((0, 0));
    let mut future = vec![];
    let mut steps = 0;
    while !active.is_empty() {
        for (x, y) in active.drain(..) {
            if (x, y) == goal {
                return Some(steps);
            }
            if seen[y][x] {
                continue;
            }
            seen[y][x] = true;
            for (xx, yy) in NeighborIterator::new(height, width, x, y, false) {
                // reduces churn
                if seen[yy][xx] {
                    continue;
                }
                future.push((xx, yy));
            }
        }
        std::mem::swap(&mut active, &mut future);
        steps += 1;
    }
    None
}

//region Iterator over neighboring tiles in a grid; four- or eight-connected.
struct NeighborIterator {
    height: isize,
    width: isize,
    x: isize,
    y: isize,
    scan_index: usize,
    connect: usize,
}

impl NeighborIterator {
    fn new(
        height: usize,
        width: usize,
        x: usize,
        y: usize,
        is_eight_connected: bool,
    ) -> NeighborIterator {
        Self {
            height: height as isize,
            width: width as isize,
            x: x as isize,
            y: y as isize,
            scan_index: 0,
            connect: 4 * (is_eight_connected as usize + 1),
        }
    }
    const OFFSETS4: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    const OFFSETS8: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        /*0,0*/ (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
}

impl Iterator for NeighborIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.scan_index < self.connect {
            let (dx, dy) = match self.connect {
                4 => NeighborIterator::OFFSETS4[self.scan_index].clone(),
                8 => NeighborIterator::OFFSETS8[self.scan_index].clone(),
                _ => unreachable!(),
            };
            self.scan_index += 1;
            let c = (self.x + dx, self.y + dy);
            let can_dx = 0 <= c.0 && c.0 < self.width;
            let can_dy = 0 <= c.1 && c.1 < self.height;
            if can_dx && can_dy {
                return Some((c.0 as usize, c.1 as usize));
            }
        }
        None
    }
}
//endregion
