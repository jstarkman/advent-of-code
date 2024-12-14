use core::str;
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
    sync::LazyLock,
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let mut claws = parse_input(BufReader::new(f));
    // dbg!(&claws);
    // Part one
    let mut p1 = 0;
    for claw in claws.iter() {
        // dbg!(claw);
        if let Some(tokens) = claw.cheapest_path() {
            p1 += tokens;
        }
    }
    dbg!(p1);
    // Part two
    let mut p2 = 0;
    const OOPS: i64 = 10000000000000;
    for claw in claws.iter_mut() {
        claw.prize.0 += OOPS;
        claw.prize.1 += OOPS;
        if let Some(tokens) = claw.cheapest_path() {
            p2 += tokens;
        }
    }
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<ClawMachine> {
    let mut claws = vec![];
    let mut buf = vec![];
    for b in r.bytes().chain(std::iter::once(Ok(b'\n'))) {
        let b = b.expect("sane input");
        if b == b'\n' && buf.last().filter(|x| **x == b'\n').is_some() {
            let claw = str::from_utf8(&buf)
                .expect("ASCII")
                .parse::<ClawMachine>()
                .expect("sane input");
            claws.push(claw);
            buf.clear();
        }
        buf.push(b);
    }
    claws
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
    cost_a: i64,
    cost_b: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ClawMachineParseError;

static REGEX_PARSE_BUTTON: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Button [AB]: X\+(\d+), Y\+(\d+)$").unwrap());
static REGEX_PARSE_PRIZE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap());
impl FromStr for ClawMachine {
    type Err = ClawMachineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let a = lines.next().ok_or(ClawMachineParseError)?;
        let a_xy = REGEX_PARSE_BUTTON
            .captures_iter(a)
            .map(|cap| cap.extract())
            .map(|(_, [x, y])| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .next()
            .ok_or(ClawMachineParseError)?;
        let b = lines.next().ok_or(ClawMachineParseError)?;
        let b_xy = REGEX_PARSE_BUTTON
            .captures_iter(b)
            .map(|cap| cap.extract())
            .map(|(_, [x, y])| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .next()
            .ok_or(ClawMachineParseError)?;
        let p = lines.next().ok_or(ClawMachineParseError)?;
        let prize = REGEX_PARSE_PRIZE
            .captures_iter(p)
            .map(|cap| cap.extract())
            .map(|(_, [x, y])| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .next()
            .ok_or(ClawMachineParseError)?;
        Ok(ClawMachine {
            button_a: a_xy,
            button_b: b_xy,
            prize,
            cost_a: 3,
            cost_b: 1,
        })
    }
}

impl ClawMachine {
    fn cheapest_path(&self) -> Option<i64> {
        // FIXME Check for linearity and numerical instabilities.
        fn determinate(v0: (i64, i64), v1: (i64, i64)) -> i64 {
            (v0.0 * v1.1) - (v0.1 * v1.0)
        }
        let det = determinate(self.button_a, self.button_b);
        let a = determinate(self.prize, self.button_b);
        let b = determinate(self.button_a, self.prize);
        if a % det == 0 && b % det == 0 {
            Some(self.cost_a * (a / det) + self.cost_b * (b / det))
        } else {
            None
        }
    }
}
