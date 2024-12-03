use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::LazyLock,
};

use regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    // manually checked that no multiplies are split across lines
    let mut from_ram = 0;
    for line in BufReader::new(f)
        .lines()
        .map(|line| line.expect("sane input"))
    {
        for (a, b) in get_muls(&line) {
            // No overflow: only have <1000 `mul`s and each is at most 1_000_000.
            from_ram += a * b;
        }
    }
    dbg!(from_ram);
    // Part two
    let f = File::open(path_input)?;
    // manually checked that no opcodes of any kind are split across lines
    let mut from_ram_filtered = 0;
    let mut muls_are_enabled = true;
    for line in BufReader::new(f)
        .lines()
        .map(|line| line.expect("sane input"))
    {
        for instruction in get_muls_part_two(&line) {
            // dbg!(&instruction);
            match instruction {
                Instruction::Mul(a, b) => {
                    if muls_are_enabled {
                        // No overflow: cannot have more to add than Part One.
                        from_ram_filtered += a * b;
                    }
                }
                Instruction::Do() => muls_are_enabled = true,
                Instruction::Dont() => muls_are_enabled = false,
            }
        }
    }
    dbg!(from_ram_filtered);
    Ok(())
}

static REGEX_GET_MULS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
fn get_muls<'a>(raw: &'a str) -> impl Iterator<Item = (i32, i32)> + 'a {
    REGEX_GET_MULS
        .captures_iter(raw)
        .map(|cap| cap.extract())
        .map(|(_, [a, b])| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
}

static REGEX_GET_MUL_DO_DONT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul|do|don't)\(((?:\d{1,3},\d{1,3})?)\)").unwrap());
static REGEX_GET_MUL_ARGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{1,3}),(\d{1,3})$").unwrap());
fn get_muls_part_two<'a>(raw: &'a str) -> impl Iterator<Item = Instruction> + 'a {
    REGEX_GET_MUL_DO_DONT
        .captures_iter(raw)
        .map(|cap| cap.extract())
        .flat_map(|(_, [opcode, args])| match (opcode, !args.is_empty()) {
            ("mul", true) => {
                fn happy_only(args: &str) -> Option<Instruction> {
                    let cap = REGEX_GET_MUL_ARGS.captures(args)?;
                    let a = cap.get(1)?.as_str().parse::<i32>().ok()?;
                    let b = cap.get(2)?.as_str().parse::<i32>().ok()?;
                    Some(Instruction::Mul(a, b))
                }
                happy_only(args)
            }
            ("do", false) => Some(Instruction::Do()),
            ("don't", false) => Some(Instruction::Dont()),
            _ => None,
        })
}

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do(),
    Dont(),
}
