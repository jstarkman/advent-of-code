use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    // Part one
    let eqs = parse_input(BufReader::new(f));
    let p1: u64 = eqs
        .iter()
        .filter_map(|eq| eq.could_be_true().then_some(eq.test_value))
        .sum();
    dbg!(p1);
    let p2: u64 = eqs
        .iter()
        .filter_map(|eq| eq.could_be_true_with_concat().then_some(eq.test_value))
        .sum();
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<Equation> {
    r.lines()
        .map(|line| line.expect("sane input"))
        .map(|line| line.as_str().into())
        .collect()
}

#[derive(Debug)]
struct Equation {
    /// Input's largest value is about 10^14.5, and there are under 1000 of them.
    test_value: u64,
    /// Input's longest list has twelve numbers.
    other_values: Vec<u64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let i = value.find(':').expect("sane input");
        let test_value = value
            .get(0..i)
            .expect("sane input")
            .parse::<u64>()
            .expect("sane input");
        let other_values: Vec<_> = value
            .get(i + 2..)
            .expect("sane input")
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().expect("sane input"))
            .collect();
        assert!(!other_values.is_empty(), "malformed input");
        Self {
            test_value,
            other_values,
        }
    }
}

impl Equation {
    pub fn could_be_true(&self) -> bool {
        Self::_could_be_true(
            self.test_value,
            self.other_values[0],
            &self.other_values[1..],
        )
    }
    fn _could_be_true(target: u64, accumulator: u64, unused: &[u64]) -> bool {
        if let Some((head, tail)) = unused.split_first() {
            // Known operators: addition, multiplication
            let add = Self::_could_be_true(target, accumulator + head, tail);
            if add {
                return true;
            }
            let mul = Self::_could_be_true(target, accumulator * head, tail);
            mul
        } else {
            target == accumulator
        }
    }

    pub fn could_be_true_with_concat(&self) -> bool {
        Self::_could_be_true_with_concat(
            self.test_value,
            self.other_values[0],
            &self.other_values[1..],
        )
    }

    fn _could_be_true_with_concat(target: u64, accumulator: u64, unused: &[u64]) -> bool {
        if let Some((&head, tail)) = unused.split_first() {
            // Known operators: addition, multiplication, concatenation
            let add = Self::_could_be_true_with_concat(target, accumulator + head, tail);
            if add {
                return true;
            }
            let mul = Self::_could_be_true_with_concat(target, accumulator * head, tail);
            if mul {
                return true;
            }
            let head_width = Self::width_as_power_of_ten(head);
            let concat =
                Self::_could_be_true_with_concat(target, accumulator * head_width + head, tail);
            concat
        } else {
            target == accumulator
        }
    }

    fn width_as_power_of_ten(x: u64) -> u64 {
        10_u64.pow((x as f64 + 0.1).log10().ceil() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(10, Equation::width_as_power_of_ten(1));
    }
    #[test]
    fn ten() {
        assert_eq!(100, Equation::width_as_power_of_ten(10));
    }
    #[test]
    fn hundred() {
        assert_eq!(1000, Equation::width_as_power_of_ten(100));
    }
    #[test]
    fn not_round_1() {
        assert_eq!(10, Equation::width_as_power_of_ten(2));
    }
    #[test]
    fn not_round_2() {
        assert_eq!(100, Equation::width_as_power_of_ten(11));
        assert_eq!(100, Equation::width_as_power_of_ten(99));
    }
}
