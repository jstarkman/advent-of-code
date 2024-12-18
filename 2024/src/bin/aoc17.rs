use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
    sync::LazyLock,
};

use regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let state = parse_input(BufReader::new(f));
    println!("{state:?}");
    let mut state_p1 = state.clone();
    state_p1.run_until_halted(state_p1.abc);
    let p1 = state_p1
        .raw_output
        .into_iter()
        .fold("".to_owned(), |mut acc, x| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push(char::from_digit(x as u32, 10).unwrap());
            acc
        });
    dbg!(p1);
    // Part two
    let raw_offset = state.raw_program.len() - 1;
    let mut state = state;
    state.abc.0 = 0;
    let my_abc = state.abc;
    let p2 = do_part_two(&mut state, my_abc, raw_offset);
    dbg!(p2);
    Ok(())
}

fn do_part_two(state: &mut ProgramState, abc: (u64, u64, u64), raw_offset: usize) -> u64 {
    if raw_offset == usize::MAX {
        return abc.0;
    }
    for i in 0..8 {
        let maybe_a = abc.0 * 8 + i;
        let maybe_abc = (maybe_a, abc.1, abc.2);
        state.run_until_halted(maybe_abc);
        // println!("Found {:?}", state.raw_output);
        if state.raw_output[0] == state.raw_program[raw_offset] {
            // println!("\tTrying {:?}", raw_offset.wrapping_sub(1));
            let aa = do_part_two(state, maybe_abc, raw_offset.wrapping_sub(1));
            if aa > 0 {
                return aa;
            }
        }
    }
    0
}

fn parse_input(mut r: BufReader<File>) -> ProgramState {
    let mut buf = vec![];
    r.read_to_end(&mut buf).unwrap();
    let s = String::from_utf8(buf).unwrap();
    s.parse().unwrap()
}

#[derive(Clone, Debug)]
struct ProgramState {
    abc: (u64, u64, u64),
    raw_program: Vec<u64>,
    pc: usize,
    raw_output: Vec<u64>,
}
impl ProgramState {
    fn new(a: u64, b: u64, c: u64, raw_program: Vec<u64>) -> Self {
        Self {
            abc: (a, b, c),
            raw_program,
            pc: 0,
            raw_output: vec![],
        }
    }

    fn run_until_halted(&mut self, abc: (u64, u64, u64)) {
        self.abc = abc;
        self.pc = 0;
        self.raw_output.clear();
        while self.step() {
            // keep going
        }
    }

    fn step(&mut self) -> bool {
        let Some(&[opcode, data]) = self.raw_program.get(self.pc..self.pc + 2) else {
            return false;
        };
        let literal = data;
        let combo = match data {
            d @ 0..=3 => d,
            4 => self.abc.0,
            5 => self.abc.1,
            6 => self.abc.2,
            7.. => panic!("reserved and will not appear in valid programs"),
        };
        const NO_WRAP: u64 = 0x3F;
        match opcode {
            0 => self.abc.0 = self.abc.0 >> (combo & NO_WRAP),
            1 => self.abc.1 ^= literal,
            2 => self.abc.1 = combo & 0x07,
            3 => {
                if self.abc.0 != 0 {
                    self.pc = literal as usize;
                    return true;
                }
            }
            4 => self.abc.1 ^= self.abc.2,
            5 => {
                let out = combo & 0x07;
                self.raw_output.push(out);
            }
            6 => self.abc.1 = self.abc.0 >> (combo & NO_WRAP),
            7 => self.abc.2 = self.abc.0 >> (combo & NO_WRAP),
            oops @ _ => panic!("bad opcode {oops}"),
        }
        self.pc += 2;
        true
    }
}

static REGEX_FROMSTR_PROGRAMSTATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)").unwrap()
});
impl FromStr for ProgramState {
    type Err = AOCParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = REGEX_FROMSTR_PROGRAMSTATE
            .captures(s)
            .ok_or(AOCParseError)?;
        let a = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .expect("numeric A");
        let b = caps
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .expect("numeric B");
        let c = caps
            .get(3)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .expect("numeric C");
        let mut raw_program = vec![];
        let p = caps.get(4).unwrap().as_str();
        for word in p.split(',') {
            let x: u64 = word.parse().unwrap();
            raw_program.push(x);
        }
        Ok(ProgramState::new(a, b, c, raw_program))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;
