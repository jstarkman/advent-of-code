// Advent of ... Sokoban?

use std::{
    fmt::{Display, Formatter, Write},
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let (map, directions) = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(&map, &directions);
    dbg!(p1);
    // Part two
    // do_part_two(&mut robots, height, width);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> (Map, Vec<Direction>) {
    let mut map = Map::default();
    let mut directions = vec![];
    let mut have_reached_directions = false;
    for line in r.lines() {
        let line = line.expect("sane input");
        if line.is_empty() {
            have_reached_directions = true;
            continue;
        }
        if have_reached_directions {
            line.bytes()
                .filter_map(|b| b.try_into().ok())
                .for_each(|dxn| directions.push(dxn));
        } else {
            let map_row: Vec<_> = line.bytes().filter_map(|b| b.try_into().ok()).collect();
            if let Some(x) = map_row
                .iter()
                .enumerate()
                .find_map(|(i, t)| (*t == MapTile::Robot).then_some(i))
            {
                map.robot_xy = (x, map.height);
            }
            map.tiles.push(map_row);
            if map.width == 0 {
                map.width = line.trim().len();
            }
            map.height += 1;
        }
    }
    (map, directions)
}

fn do_part_one(map: &Map, directions: &[Direction]) -> u64 {
    let mut map = map.clone();
    for &dxn in directions.iter() {
        map.try_move(dxn);
        // println!("{map}");
    }
    map.box_gps_total()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapTile {
    Open,
    Box,
    Wall,
    Robot,
}

impl TryFrom<u8> for MapTile {
    type Error = AOCParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(MapTile::Open),
            b'O' | b'V' => Ok(MapTile::Box),
            b'#' => Ok(MapTile::Wall),
            b'@' => Ok(MapTile::Robot),
            _ => Err(AOCParseError),
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            MapTile::Open => '.',
            MapTile::Box => 'O',
            MapTile::Wall => '#',
            MapTile::Robot => '@',
        })
    }
}

#[derive(Clone, Debug, Default)]
struct Map {
    tiles: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    robot_xy: (usize, usize),
}

impl Map {
    fn try_move(&mut self, dxn: Direction) {
        let (x, y) = self.robot_xy;
        let (dx, dy) = dxn.as_wrapping_dxdy();
        // "h" for "hypothetical"
        let (hx, hy) = (x.wrapping_add(dx), y.wrapping_add(dy));
        // Safety: all inputs are surrounded in walls, so all four neighbors of `robot_xy` are always legal indices.
        match self.tiles[hy][hx] {
            MapTile::Open => {
                self.tiles[y][x] = MapTile::Open;
                self.tiles[hy][hx] = MapTile::Robot;
                self.robot_xy = (hx, hy);
            }
            MapTile::Box => {
                // Only legal if not blocked by a wall.  The robot is infinitely strong; no "Sokoban" limit.
                // "b" for "box's x/y"
                let (mut bx, mut by) = (hx, hy);
                // Safety: same as above; all inputs are surrounded in walls, so all neighbors are always legal.
                while self.tiles[by][bx] == MapTile::Box {
                    (bx, by) = (bx.wrapping_add(dx), by.wrapping_add(dy));
                }
                match self.tiles[by][bx] {
                    MapTile::Open => {
                        // Can move; similar to the usual `Open` case, but with a moved box.
                        self.tiles[by][bx] = MapTile::Box;
                        self.tiles[hy][hx] = MapTile::Robot;
                        self.tiles[y][x] = MapTile::Open;
                        self.robot_xy = (hx, hy);
                    }
                    MapTile::Box => unreachable!("while == Box, above"),
                    MapTile::Wall => (),
                    MapTile::Robot => panic!("really cannot have two robots"),
                }
            }
            MapTile::Wall => (),
            MapTile::Robot => panic!("cannot have two robots"),
        }
    }

    fn box_gps_total(&self) -> u64 {
        let mut total_gps = 0_u64;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y][x] == MapTile::Box {
                    let gps = y * 100 + x;
                    total_gps += gps as u64;
                }
            }
        }
        total_gps
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for scanline in self.tiles.iter() {
            for t in scanline.iter() {
                t.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = AOCParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Direction::Up),
            b'v' | b'V' => Ok(Direction::Down),
            b'<' => Ok(Direction::Left),
            b'>' => Ok(Direction::Right),
            _ => Err(AOCParseError),
        }
    }
}

impl Direction {
    fn as_wrapping_dxdy(&self) -> (usize, usize) {
        match self {
            Direction::Up => (0, usize::MAX),
            Direction::Down => (0, 1),
            Direction::Left => (usize::MAX, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;
