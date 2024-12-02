use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let safe_reports = BufReader::new(f)
        .lines()
        .map(|line| line.expect("sane input"))
        .filter(|line| is_report_safe(line))
        .count();
    dbg!(safe_reports);
    // Part two
    let f = File::open(path_input)?;
    let safe_damped_reports = BufReader::new(f)
        .lines()
        .map(|line| line.expect("sane input"))
        .filter(|line| is_damped_report_safe(line))
        .count();
    dbg!(safe_damped_reports);
    Ok(())
}

fn is_report_safe(line: &str) -> bool {
    let iter_levels = line
        .split_ascii_whitespace()
        .map(|level| level.parse::<i8>().expect("sane input"));
    are_levels_safe(iter_levels)
}

fn are_levels_safe<I>(iter_levels: I) -> bool
where
    I: Iterator<Item = i8>,
{
    let mut iter_levels = iter_levels.peekable();
    let baseline: i8 = iter_levels.next().expect("sane input");
    let mut prev = baseline;
    let is_inc = prev < *iter_levels.peek().expect("sane input");
    for level in iter_levels {
        let d = (level - prev) * (is_inc as i8 * 2 - 1);
        if !(1..=3).contains(&d) {
            // println!("{d} == {level} - {prev}");
            return false;
        }
        prev = level;
    }
    true
}

fn is_damped_report_safe(line: &str) -> bool {
    if is_report_safe(line) {
        return true; // majority of cases
    }
    // If not, we do a literal, brute-force interpretation of the prompt.
    let levels: Vec<_> = line
        .split_ascii_whitespace()
        .map(|level| level.parse::<i8>().expect("sane input"))
        .collect();
    for i in 0..levels.len() {
        let head = &levels[..i];
        let tail = &levels[i + 1..];
        let pseudo_report = head.iter().chain(tail.iter()).cloned();
        // println!("{head:?} + {tail:?}");
        if are_levels_safe(pseudo_report) {
            // println!("safe");
            return true;
        }
    }
    false
}
