// Advent of ... Sokoban?

use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Write},
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let (map, directions) = parse_input(BufReader::new(f), false);
    let p1 = do_either_part(map, &directions);
    dbg!(p1);
    // Part two
    let f = File::open(path_input)?;
    let (map, directions) = parse_input(BufReader::new(f), true);
    let p2 = do_either_part(map, &directions);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>, is_double_width: bool) -> (Map, Vec<Direction>) {
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
            let mut map_row = vec![];
            for b in line.bytes() {
                if is_double_width {
                    let [l, r] = MapTile::try_from_2x(b).unwrap();
                    map_row.push(l);
                    map_row.push(r);
                } else {
                    let s = b.try_into().unwrap();
                    map_row.push(s);
                }
            }
            if let Some(x) = map_row
                .iter()
                .enumerate()
                .find_map(|(i, t)| (*t == MapTile::Robot).then_some(i))
            {
                map.robot_xy = (x, map.height);
            }
            map.tiles.push(map_row);
            if map.width == 0 {
                let width = line.trim().len();
                let width = if is_double_width {
                    width + width
                } else {
                    width
                };
                map.width = width;
            }
            map.height += 1;
        }
    }
    (map, directions)
}

fn do_either_part(mut map: Map, directions: &[Direction]) -> u64 {
    for &dxn in directions.iter() {
        let _did_move = map.try_move(dxn);
        // println!("{map}");
    }
    println!("{map}");
    map.box_gps_total()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoxType {
    Single,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapTile {
    Open,
    Box(BoxType),
    Wall,
    Robot,
}

impl MapTile {
    fn try_from_2x(value: u8) -> Result<[Self; 2], AOCParseError> {
        match value {
            b'.' => Ok([MapTile::Open, MapTile::Open]),
            b'O' | b'V' => Ok([MapTile::Box(BoxType::Left), MapTile::Box(BoxType::Right)]),
            b'#' => Ok([MapTile::Wall, MapTile::Wall]),
            b'@' => Ok([MapTile::Robot, MapTile::Open]),
            _ => Err(AOCParseError),
        }
    }
}

impl TryFrom<u8> for MapTile {
    type Error = AOCParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(MapTile::Open),
            b'O' | b'V' => Ok(MapTile::Box(BoxType::Single)),
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
            MapTile::Box(BoxType::Single) => 'O',
            MapTile::Box(BoxType::Left) => '[',
            MapTile::Box(BoxType::Right) => ']',
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
    fn try_move(&mut self, direction: Direction) -> bool {
        let (x, y) = self.robot_xy;
        let (dx, dy) = direction.as_wrapping_dxdy();
        // "h" for "hypothetical"
        let (hx, hy) = (x.wrapping_add(dx), y.wrapping_add(dy));
        // Safety: all inputs are surrounded in walls, so all four neighbors of `robot_xy` are always legal indices.
        match self.tiles[hy][hx] {
            MapTile::Open => {
                self.tiles[y][x] = MapTile::Open;
                self.tiles[hy][hx] = MapTile::Robot;
                self.robot_xy = (hx, hy);
                true
            }
            MapTile::Box(BoxType::Single) => {
                // Only legal if not blocked by a wall.  The robot is infinitely strong; no "Sokoban" limit.
                // "b" for "box's x/y"
                let (mut bx, mut by) = (hx, hy);
                // Safety: same as above; all inputs are surrounded in walls, so all neighbors are always legal.
                while self.tiles[by][bx] == MapTile::Box(BoxType::Single) {
                    (bx, by) = (bx.wrapping_add(dx), by.wrapping_add(dy));
                }
                match self.tiles[by][bx] {
                    MapTile::Open => {
                        // Can move; similar to the usual `Open` case, but with a moved box.
                        self.tiles[by][bx] = MapTile::Box(BoxType::Single);
                        self.tiles[hy][hx] = MapTile::Robot;
                        self.tiles[y][x] = MapTile::Open;
                        self.robot_xy = (hx, hy);
                        true
                    }
                    MapTile::Box(BoxType::Single) => unreachable!("while == Box, above"),
                    MapTile::Box(_) => panic!("shall not mix BoxType::Single with the others"),
                    MapTile::Wall => false,
                    MapTile::Robot => panic!("really cannot have two robots"),
                }
            }
            MapTile::Box(lr @ _) => {
                // (x,y) of the left box.
                let mut coords = HashSet::new();
                fn can_move(
                    // Same for all recursion levels
                    coords: &mut HashSet<(usize, usize)>,
                    tiles: &Vec<Vec<MapTile>>,
                    direction: Direction,
                    // Changes
                    (x, y): (usize, usize),
                ) -> bool {
                    assert_eq!(
                        tiles[y][x],
                        MapTile::Box(BoxType::Left),
                        "always left; got ({x},{y}) while going {direction:?}",
                    );
                    let (dx, dy) = direction.as_wrapping_dxdy();
                    let (hx, hy) = (x.wrapping_add(dx), y.wrapping_add(dy));
                    let able_to_move = match direction {
                        Direction::Up | Direction::Down => {
                            match (tiles[hy][hx], tiles[hy][hx + 1]) {
                                (MapTile::Robot, _) | (_, MapTile::Robot) => panic!(
                                    "only have one robot; and it cannot reach around behind itself"
                                ),
                                (MapTile::Wall, _) | (_, MapTile::Wall) => false,
                                (MapTile::Open, MapTile::Open) => true,
                                (MapTile::Box(BoxType::Right), MapTile::Open) => {
                                    can_move(coords, tiles, direction, (hx - 1, hy))
                                }
                                (MapTile::Box(BoxType::Left), MapTile::Box(BoxType::Right)) => {
                                    can_move(coords, tiles, direction, (hx, hy))
                                }
                                (MapTile::Open, MapTile::Box(BoxType::Left)) => {
                                    can_move(coords, tiles, direction, (hx + 1, hy))
                                }
                                (MapTile::Box(BoxType::Right), MapTile::Box(BoxType::Left)) => {
                                    can_move(coords, tiles, direction, (hx - 1, hy))
                                        && can_move(coords, tiles, direction, (hx + 1, hy))
                                }
                                _ => panic!("broken boxes"),
                            }
                        }
                        Direction::Left => match tiles[hy][hx] {
                            MapTile::Open => true,
                            MapTile::Box(BoxType::Right) => {
                                can_move(coords, tiles, direction, (hx - 1, hy))
                            }
                            MapTile::Box(_) => panic!("broken box"),
                            MapTile::Wall => false,
                            MapTile::Robot => panic!(
                                "only have one robot; and it cannot reach around behind itself"
                            ),
                        },
                        Direction::Right => match tiles[hy][hx + 1] {
                            MapTile::Open => true,
                            MapTile::Box(BoxType::Left) => {
                                can_move(coords, tiles, direction, (hx + 1, hy))
                            }
                            MapTile::Box(_) => panic!("broken box"),
                            MapTile::Wall => false,
                            MapTile::Robot => panic!(
                                "only have one robot; and it cannot reach around behind itself"
                            ),
                        },
                    };
                    if able_to_move {
                        coords.insert((x, y));
                    }
                    able_to_move
                }
                // "p" for "pushed"
                let (px, py) = if lr == BoxType::Left {
                    (hx, hy)
                } else {
                    (hx - 1, hy)
                };
                if can_move(&mut coords, &self.tiles, direction, (px, py)) {
                    // Do the move: delete old, then write new; "i" for "iteration box".
                    for &(ix, iy) in coords.iter() {
                        self.tiles[iy][ix] = MapTile::Open;
                        self.tiles[iy][ix + 1] = MapTile::Open;
                    }
                    for &(ix, iy) in coords.iter() {
                        let (bx, by) = (ix.wrapping_add(dx), iy.wrapping_add(dy));
                        self.tiles[by][bx] = MapTile::Box(BoxType::Left);
                        self.tiles[by][bx + 1] = MapTile::Box(BoxType::Right);
                    }
                    // Then move the robot; similar to the usual `Open` case, but with a moved box.
                    self.tiles[y][x] = MapTile::Open;
                    self.tiles[hy][hx] = MapTile::Robot;
                    self.robot_xy = (hx, hy);
                    true
                } else {
                    false
                }
            }
            MapTile::Wall => false,
            MapTile::Robot => panic!("cannot have two robots"),
        }
    }

    fn box_gps_total(&self) -> u64 {
        let mut total_gps = 0_u64;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles[y][x] {
                    MapTile::Box(BoxType::Single) | MapTile::Box(BoxType::Left) => {
                        let gps = y * 100 + x;
                        total_gps += gps as u64;
                    }
                    _ => (),
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
