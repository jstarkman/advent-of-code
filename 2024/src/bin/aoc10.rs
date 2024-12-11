use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let map = parse_input(BufReader::new(f));
    // Part one
    let p1 = map.do_part_one();
    dbg!(p1);
    // Part two
    let p2 = map.do_part_two();
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Map {
    let mut buf = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in r.lines() {
        let line = line.expect("sane");
        width = line.len(); // both inputs are of constant width
        for b in line.bytes() {
            buf.push(b);
        }
        height += 1;
    }
    Map {
        tiles: buf,
        width,
        height,
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn do_part_one(&self) -> u64 {
        let trailheads = self.buffer_trailheads();
        // println!("{trailheads:?}");
        // parallel if needed
        let mut trails = 0;
        for (x, y) in trailheads.into_iter() {
            let mut tails: HashSet<(usize, usize)> = HashSet::new();
            self.find_trailtails((x, y), &mut tails);
            let t = tails.len();
            // println!("{x}, {y}\t{t}");
            trails += t as u64;
        }
        trails
    }

    fn do_part_two(&self) -> u64 {
        let trailheads = self.buffer_trailheads();
        // println!("{trailheads:?}");
        // parallel if needed
        let mut trails = 0;
        for (x, y) in trailheads.into_iter() {
            let t = self.count_distinct_trails((x, y));
            // println!("{x}, {y}\t{t}");
            trails += t as u64;
        }
        trails
    }

    fn buffer_trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = vec![];
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[i] == b'0' {
                    trailheads.push((x, y));
                }
                i += 1;
            }
        }
        trailheads
    }

    fn find_trailtails(&self, (x, y): (usize, usize), tails: &mut HashSet<(usize, usize)>) {
        let altitude = self.tiles[y * self.width + x];
        if altitude == b'9' {
            tails.insert((x, y));
            return;
        }
        for (xx, yy) in NeighborIterator::new(self.height, self.width, x, y, false) {
            let aa = self.tiles[yy * self.width + xx];
            if altitude + 1 == aa {
                self.find_trailtails((xx, yy), tails);
            }
        }
    }

    fn count_distinct_trails(&self, (x, y): (usize, usize)) -> u64 {
        let altitude = self.tiles[y * self.width + x];
        if altitude == b'9' {
            return 1;
        }
        let mut trails = 0;
        for (xx, yy) in NeighborIterator::new(self.height, self.width, x, y, false) {
            let aa = self.tiles[yy * self.width + xx];
            if altitude + 1 == aa {
                trails += self.count_distinct_trails((xx, yy));
            }
        }
        trails
    }
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
