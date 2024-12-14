use core::str;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    sync::LazyLock,
    time::Duration,
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let (height, width) = if path_input.contains("mini") {
        (7, 11)
    } else {
        (103, 101)
    };
    let mut robots = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(&robots, height, width, 100);
    dbg!(p1);
    // Part two
    do_part_two(&mut robots, height, width);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<Robot> {
    let mut robots = vec![];
    for line in r.lines() {
        let line = line.expect("sane input");
        let robot = line.parse::<Robot>().expect("sane input");
        robots.push(robot);
    }
    robots
}

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

#[derive(Debug, PartialEq, Eq)]
struct RobotParseError;

static REGEX_PARSE_ROBOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap());
impl FromStr for Robot {
    type Err = RobotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim();
        let robot = REGEX_PARSE_ROBOT
            .captures_iter(raw)
            .map(|cap| cap.extract())
            .map(|(_, [px, py, vx, vy])| Robot {
                position: (px.parse::<i64>().unwrap(), py.parse::<i64>().unwrap()),
                velocity: (vx.parse::<i64>().unwrap(), vy.parse::<i64>().unwrap()),
            })
            .next()
            .ok_or(RobotParseError)?;
        Ok(robot)
    }
}

fn do_part_one(robots: &[Robot], height: i64, width: i64, steps: u32) -> u64 {
    let (half_height, half_width) = (height / 2, width / 2);
    let (mut quad_nw, mut quad_ne, mut quad_sw, mut quad_se) = (0, 0, 0, 0);
    for robot in robots.iter() {
        let (mut x, mut y) = robot.position;
        // Alternatively, do pos + (steps * vel) % width and do not worry about overflows.
        for _ in 0..steps {
            x = (x + robot.velocity.0 + width) % width;
            y = (y + robot.velocity.1 + height) % height;
        }
        use std::cmp::Ordering;
        match (x.cmp(&half_width), y.cmp(&half_height)) {
            // "Robots that are exactly in the middle (horizontally or vertically) don't count as being in any quadrant"
            (_, Ordering::Equal) => (),
            (Ordering::Equal, _) => (),
            // The rest do count.
            (Ordering::Less, Ordering::Less) => quad_nw += 1,
            (Ordering::Less, Ordering::Greater) => quad_sw += 1,
            (Ordering::Greater, Ordering::Less) => quad_ne += 1,
            (Ordering::Greater, Ordering::Greater) => quad_se += 1,
        }
    }
    quad_nw * quad_ne * quad_sw * quad_se
}

fn do_part_two(robots: &mut [Robot], height: i64, width: i64) {
    for i in 0.. {
        let world = render_the_world(robots, height, width);
        // heuristic
        if world.contains("#####") {
            println!("\t{i}\n{}", world);
        }
        step_robots(robots, height, width);
        std::thread::sleep(Duration::from_millis(10));
        // Final answer: 7132
    }
}

fn step_robots(robots: &mut [Robot], height: i64, width: i64) {
    for robot in robots.iter_mut() {
        robot.position.0 = (robot.position.0 + robot.velocity.0 + width) % width;
        robot.position.1 = (robot.position.1 + robot.velocity.1 + height) % height;
    }
}

fn render_the_world(robots: &[Robot], height: i64, width: i64) -> String {
    // FIXME convert to flat vector, like in C
    let mut print_me = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots.iter() {
        let (x, y) = (robot.position.0 as usize, robot.position.1 as usize);
        print_me[y][x] = '#';
    }
    String::from_iter(
        print_me
            .into_iter()
            .flat_map(|line| line.into_iter().chain(std::iter::once('\n'))),
    )
}
