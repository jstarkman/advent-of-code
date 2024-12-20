use std::{
    fmt::{Display, Formatter, Write},
    fs::File,
    io::{BufRead, BufReader},
    sync::LazyLock,
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let map = parse_input(BufReader::new(f));
    let distances_start = map.distances_from_start();
    let distances_end = map.distances_from_end();
    // let path = map.shortest_path(&distances_start);
    // println!("{path:?}");
    // Part one
    let p1 = map.good_cheats(&distances_start, &distances_end, 2, 100);
    dbg!(p1);
    // Part two
    let p2 = map.good_cheats(&distances_start, &distances_end, 20, 100);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Map {
    let mut tiles = vec![];
    let mut start_xy = (0, 0);
    let mut end_xy = (0, 0);
    for (y, line) in r.lines().flat_map(|l| l.ok().into_iter()).enumerate() {
        let mut map_row = vec![];
        for (x, b) in line.bytes().enumerate() {
            let s = b.try_into().unwrap();
            match s {
                MapTile::Wall | MapTile::Open => (),
                MapTile::Start => start_xy = (x, y),
                MapTile::End => end_xy = (x, y),
            }
            map_row.push(s);
        }
        tiles.push(map_row);
    }
    let width = tiles[0].len();
    let height = tiles.len();
    Map {
        tiles,
        width,
        height,
        start_xy,
        end_xy,
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    start_xy: (usize, usize),
    end_xy: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapTile {
    Wall,
    Open,
    Start,
    End,
}

impl TryFrom<u8> for MapTile {
    type Error = AOCParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(MapTile::Wall),
            b'.' => Ok(MapTile::Open),
            b'S' => Ok(MapTile::Start),
            b'E' => Ok(MapTile::End),
            _ => Err(AOCParseError),
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            MapTile::Wall => '#',
            MapTile::Open => '.',
            MapTile::Start => 'S',
            MapTile::End => 'E',
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;

impl Map {
    fn distances_from_start(&self) -> Vec<Vec<u32>> {
        self.distances_from(self.start_xy)
    }

    fn distances_from_end(&self) -> Vec<Vec<u32>> {
        self.distances_from(self.end_xy)
    }

    fn distances_from(&self, xy: (usize, usize)) -> Vec<Vec<u32>> {
        let mut distances = vec![vec![0; self.width]; self.height];
        for (t_row, d_row) in self.tiles.iter().zip(distances.iter_mut()) {
            for (t, d) in t_row.iter().zip(d_row.iter_mut()) {
                if *t == MapTile::Wall {
                    *d = u32::MAX;
                }
            }
        }
        let mut active = vec![];
        active.push(xy);
        let mut future = vec![];
        let mut steps = 0;
        while !active.is_empty() {
            for (x, y) in active.drain(..) {
                if distances[y][x] > 0 {
                    continue;
                }
                distances[y][x] = steps;
                for (xx, yy) in NeighborIterator::new(self.height, self.width, x, y, false) {
                    // reduces churn
                    if distances[yy][xx] > 0 {
                        continue;
                    }
                    future.push((xx, yy));
                }
            }
            std::mem::swap(&mut active, &mut future);
            steps += 1;
        }
        distances[xy.1][xy.0] = 0; // avoids condition in loop
        distances
    }

    /// Includes both ends
    #[allow(dead_code)]
    fn shortest_path(&self, distances: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
        let mut retval = vec![self.end_xy];
        loop {
            let (x, y) = retval.last().unwrap().clone();
            let d = distances[y][x];
            if d == 0 {
                break;
            }
            for (xx, yy) in NeighborIterator::new(self.height, self.width, x, y, false) {
                let dd = distances[yy][xx];
                if d - 1 == dd {
                    retval.push((xx, yy));
                    break;
                }
            }
        }
        retval.reverse();
        retval
    }

    fn good_cheats(
        &self,
        distances_start: &Vec<Vec<u32>>,
        distances_end: &Vec<Vec<u32>>,
        cheat_distance: usize,
        min_savings: u32,
    ) -> u64 {
        let mut good_cheats = 0;
        let distance_no_cheat = distances_start[self.end_xy.1][self.end_xy.0];
        for y in 0..self.height {
            for x in 0..self.width {
                let (t, ds) = (self.tiles[y][x], distances_start[y][x]);
                if t == MapTile::Wall {
                    continue;
                }
                self.for_each_nonwall_within((x, y), cheat_distance, |xx, yy| {
                    let de = distances_end[yy][xx];
                    let dx = x.max(xx) - x.min(xx);
                    let dy = y.max(yy) - y.min(yy);
                    let d = (dx + dy) as u32;
                    let distance_with_cheat = ds + d + de;
                    if distance_with_cheat + min_savings <= distance_no_cheat {
                        good_cheats += 1;
                    }
                });
            }
        }
        good_cheats
    }

    fn for_each_nonwall_within<F>(&self, xy: (usize, usize), distance: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        let dxdy = match distance {
            2 => &DISTANCE_L1_2,
            20 => &DISTANCE_L1_20,
            _ => panic!("not cached"),
        };
        let (x, y) = (xy.0 as isize, xy.1 as isize);
        for &(dx, dy) in dxdy.iter() {
            let (xx, yy) = (x + dx, y + dy);
            if xx < 0 || yy < 0 {
                continue;
            }
            let (xx, yy) = (xx as usize, yy as usize);
            if let Some(row) = self.tiles.get(yy) {
                if let Some(t) = row.get(xx) {
                    if *t != MapTile::Wall {
                        f(xx, yy);
                    }
                }
            }
        }
    }
}

static DISTANCE_L1_2: LazyLock<Vec<(isize, isize)>> = LazyLock::new(|| distance_l1(2));
static DISTANCE_L1_20: LazyLock<Vec<(isize, isize)>> = LazyLock::new(|| distance_l1(20));
fn distance_l1(radius: isize) -> Vec<(isize, isize)> {
    let mut xy = vec![];
    let radius = radius as isize;
    for y in -radius..=radius {
        for x in -radius..=radius {
            let d = y.abs() + x.abs();
            if 0 < d && d <= radius {
                xy.push((x, y));
            }
        }
    }
    // println!("{radius}\n\t{xy:?}");
    xy
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
