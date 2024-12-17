use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Formatter, Write},
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let map = parse_input(BufReader::new(f));
    let best_path_cost = find_cheapest_path(&map).expect("path should exist");
    dbg!(best_path_cost);
    // Part two
    let p2 = do_part_two(&map);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Map {
    let mut tiles = vec![];
    for line in r.lines() {
        let line = line.expect("sane input");
        let mut map_row = vec![];
        for b in line.bytes() {
            let s = b.try_into().unwrap();
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
        start_xy: (1, height - 2),
        start_direction: Direction::East,
        end_xy: (width - 2, 1),
        cost_straight: 1,
        cost_turn: 1000,
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    start_xy: (usize, usize),
    start_direction: Direction,
    end_xy: (usize, usize),
    cost_straight: u32,
    cost_turn: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapTile {
    Open,
    Wall,
}

impl TryFrom<u8> for MapTile {
    type Error = AOCParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' | b'S' | b'E' => Ok(MapTile::Open),
            b'#' => Ok(MapTile::Wall),
            _ => Err(AOCParseError),
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            MapTile::Open => '.',
            MapTile::Wall => '#',
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[allow(dead_code)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn as_wrapping_dxdy(&self) -> (usize, usize) {
        match self {
            Direction::North => (0, usize::MAX),
            Direction::South => (0, 1),
            Direction::West => (usize::MAX, 0),
            Direction::East => (1, 0),
        }
    }

    fn as_bitmask(&self) -> u8 {
        1 << (*self as u32)
    }

    const ENUM_VARIANTS: u8 = 4;
    fn iter() -> impl Iterator<Item = Direction> {
        // Safety: as long as `ENUM_VARIANTS` is accurate; relies on `repr(u8)`
        (0..Self::ENUM_VARIANTS).map(|e| unsafe { std::mem::transmute(e) })
    }

    fn both_turns(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StateAStar {
    cost: u32,
    x: usize,
    y: usize,
    d: Direction,
}

fn find_cheapest_path(map: &Map) -> Option<u32> {
    let mut seen = vec![vec![0_u8; map.width]; map.height];
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(StateAStar {
        cost: 0,
        x: map.start_xy.0,
        y: map.start_xy.1,
        d: map.start_direction,
    }));
    while let Some(Reverse(StateAStar { cost, x, y, d })) = min_heap.pop() {
        if map.end_xy == (x, y) {
            return Some(cost);
        }
        let mask = d.as_bitmask();
        if seen[y][x] & mask != 0 {
            continue;
        }
        seen[y][x] |= mask;
        // Safety: entire map is surrounded with Walls.
        let (dx, dy) = d.as_wrapping_dxdy();
        // Straight
        let (xx, yy) = (x.wrapping_add(dx), y.wrapping_add(dy));
        if map.tiles[yy][xx] == MapTile::Open {
            // Not needed, but reduces churn.
            if seen[yy][xx] & mask == 0 {
                min_heap.push(Reverse(StateAStar {
                    cost: cost + map.cost_straight,
                    x: xx,
                    y: yy,
                    d,
                }));
            }
        }
        // Rotations
        for dd in Direction::iter() {
            if d == dd {
                continue;
            }
            let maskmask = dd.as_bitmask();
            // also reduces churn
            if seen[y][x] & maskmask != 0 {
                continue;
            }
            min_heap.push(Reverse(StateAStar {
                cost: cost + map.cost_turn,
                x,
                y,
                d: dd,
            }));
        }
    }
    None
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StateAStarWithHistory {
    cost: u32,
    d: Direction,
    history: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Where {
    x: usize,
    y: usize,
    d: Direction,
}

fn do_part_two(map: &Map) -> u64 {
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(StateAStarWithHistory {
        cost: 0,
        d: map.start_direction,
        history: vec![map.start_xy],
    }));
    let mut best_costs = HashMap::new();
    best_costs.insert(
        Where {
            x: map.start_xy.0,
            y: map.start_xy.1,
            d: map.start_direction,
        },
        0,
    );
    let mut best_path_cost = u32::MAX;
    let mut best_seats = HashSet::new();
    while let Some(Reverse(StateAStarWithHistory { cost, d, history })) = min_heap.pop() {
        let (x, y) = *history.last().expect("initialized");
        if (x, y) == map.end_xy {
            best_path_cost = cost;
            for &(xx, yy) in history.iter() {
                best_seats.insert((xx, yy));
            }
            continue;
        }
        if cost >= best_path_cost {
            // not `break` because of "if end" above
            continue;
        }
        // Now, we either have not found the end; or have, but `cost` is too low.
        let mut do_next = |new_cost, xx, yy, dd| {
            let row: &Vec<MapTile> = &map.tiles[yy];
            if row[xx] != MapTile::Open {
                return;
            }
            let p = Where {
                x: xx,
                y: yy,
                d: dd,
            };
            let costcost = best_costs.entry(p).or_insert(u32::MAX);
            if new_cost <= *costcost {
                *costcost = new_cost;
                let mut history = history.clone();
                history.push((xx, yy));
                min_heap.push(Reverse(StateAStarWithHistory {
                    cost: new_cost,
                    d: dd,
                    history,
                }));
            }
        };
        let (dx, dy) = d.as_wrapping_dxdy();
        let (x_s, y_s) = (x.wrapping_add(dx), y.wrapping_add(dy));
        do_next(cost + map.cost_straight, x_s, y_s, d);
        let d_turn = d.both_turns();
        do_next(cost + map.cost_turn, x, y, d_turn[0]);
        do_next(cost + map.cost_turn, x, y, d_turn[1]);
    }
    best_seats.len() as u64
}
