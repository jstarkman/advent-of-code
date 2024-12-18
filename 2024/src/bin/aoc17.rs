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
    let mut state = parse_input(BufReader::new(f));
    println!("{state:?}");
    state.run_until_halted();
    let p1 = state.output;
    dbg!(p1);
    // Part two
    // let p2 = do_part_two(&map);
    // dbg!(p2);
    Ok(())
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
    program: Vec<Instruction>,
    /// Index into `program`; is half of problem statement's values.
    pc: usize,
    output: String,
}
impl ProgramState {
    fn run_until_halted(&mut self) {
        while self.step() {
            // keep going
        }
    }

    fn step(&mut self) -> bool {
        let Some(&Instruction(opcode, data)) = self.program.get(self.pc) else {
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
            Opcode::Adv => self.abc.0 = self.abc.0 >> (combo & NO_WRAP),
            Opcode::Bxl => self.abc.1 ^= literal,
            Opcode::Bst => self.abc.1 = combo & 0x07,
            Opcode::Jnz => {
                if self.abc.0 != 0 {
                    // Half because we group opcodes with their data.
                    if literal % 2 == 1 {
                        panic!("bad model");
                    }
                    self.pc = (literal / 2) as usize;
                    return true;
                }
            }
            Opcode::Bxc => self.abc.1 ^= self.abc.2,
            Opcode::Out => {
                if !self.output.is_empty() {
                    self.output.push(',');
                }
                let out = combo & 0x07;
                self.output.push_str(&out.to_string());
            }
            Opcode::Bdv => self.abc.1 = self.abc.0 >> (combo & NO_WRAP),
            Opcode::Cdv => self.abc.2 = self.abc.0 >> (combo & NO_WRAP),
        }
        self.pc += 1;
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
        let mut program = vec![];
        let raw_program = caps.get(4).unwrap().as_str();
        let mut opcode = None;
        for word in raw_program.split(',') {
            let x: u64 = word.parse().unwrap();
            if let Some(opcode) = opcode.take() {
                assert!(
                    (0..=6).contains(&x),
                    "Combo operand 7 is reserved and will not appear in valid programs."
                );
                program.push(Instruction(opcode, x));
            } else {
                opcode = Some(x.try_into().expect("sane input"));
            }
        }
        Ok(ProgramState {
            abc: (a, b, c),
            program,
            pc: 0,
            output: String::new(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)] // constructed via transmute()
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Opcode {
    const ENUM_VARIANTS: u64 = 8;
}

impl TryFrom<u64> for Opcode {
    type Error = AOCParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if (0..Opcode::ENUM_VARIANTS).contains(&value) {
            unsafe { Ok(std::mem::transmute(value as u8)) }
        } else {
            Err(AOCParseError)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Opcode, u64);

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;
