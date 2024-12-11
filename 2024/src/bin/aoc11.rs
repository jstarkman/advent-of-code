use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let stones = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(&stones);
    dbg!(p1);
    // Part two
    let p2 = do_part_two(&stones);
    dbg!(p2);
    Ok(())
}

fn parse_input(mut r: BufReader<File>) -> Vec<String> {
    let mut buf = String::new();
    r.read_to_string(&mut buf).unwrap();
    buf.split_ascii_whitespace().map(|s| s.to_owned()).collect()
}

fn do_part_one(stones: &[String]) -> usize {
    let mut active = stones.to_vec();
    let mut future = vec![];
    for _blink in 0..25 {
        future.clear();
        for st in active.drain(..) {
            let change = apply_rule(&st);
            change
                .into_iter()
                .filter_map(|st| st)
                .for_each(|st| future.push(st));
        }
        // println!("{_blink}\t{:?}", future.len());
        std::mem::swap(&mut active, &mut future);
    }
    active.len()
}

fn apply_rule(x: &str) -> [Option<String>; 2] {
    if x == "0" {
        return [Some("1".to_owned()), None];
    }
    if x.len() % 2 == 0 {
        let (a, b) = x.split_at(x.len() / 2);
        let a = a.trim_start_matches('0');
        let b = b.trim_start_matches('0');
        return [
            Some((if a.is_empty() { "0" } else { a }).to_owned()),
            Some((if b.is_empty() { "0" } else { b }).to_owned()),
        ];
    }
    let xx = multiply_strings(x, "2024");
    [Some(xx), None]
}

fn multiply_strings(s1: &str, s2: &str) -> String {
    if s1 == "0" || s2 == "0" {
        return "0".to_owned();
    }
    let mut prod = vec![0; s1.len() + s2.len()];
    for (i1, b1) in s1.bytes().rev().enumerate() {
        let d1 = b1 - b'0';
        for (i2, b2) in s2.bytes().rev().enumerate() {
            let d2 = b2 - b'0';
            let p = d1 * d2;
            prod[i1 + i2] += p;

            for idx in i1 + i2..prod.len() {
                let d = prod[idx];
                if d < 10 {
                    break;
                }
                prod[idx] = d % 10;
                prod[idx + 1] += d / 10;
            }
        }
        // println!("\t {prod:?}");
    }
    while *prod.last().unwrap() == 0 {
        let _leading_zero = prod.pop();
    }
    // println!("prod {prod:?}");
    prod.reverse();
    prod.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn m() {
        assert_eq!("98765432", multiply_strings("8", "12345679"));
    }

    #[test]
    fn m1() {
        assert_eq!("8", multiply_strings("2", "4"));
    }

    #[test]
    fn m2() {
        assert_eq!("18", multiply_strings("2", "9"));
    }

    #[test]
    fn m2024() {
        assert_eq!("20240", multiply_strings("10", "2024"));
    }

    #[test]
    fn m202411() {
        assert_eq!("22264", multiply_strings("11", "2024"));
    }
}

fn do_part_two(stones: &[String]) -> u64 {
    let mut cache = HashMap::new();
    let mut retval = 0;
    for st in stones {
        let st = st.parse::<u64>().unwrap();
        retval += do_part_two_rec(&mut cache, 0, st);
    }
    retval
}

const PART_TWO_LIMIT: u64 = 75;
fn do_part_two_rec(cache: &mut HashMap<(u64, u64), u64>, iteration: u64, count: u64) -> u64 {
    let cache_key = (iteration, count);
    if let Some(&retval) = cache.get(&cache_key) {
        return retval;
    }
    if iteration >= PART_TWO_LIMIT {
        return 1;
    }
    if count == 0 {
        return do_part_two_rec(cache, iteration + 1, 1);
    }
    let w = width(count);
    let retval = if w % 2 == 0 {
        let places = 10_u64.pow(w / 2);
        let a = do_part_two_rec(cache, iteration + 1, count / places);
        let b = do_part_two_rec(cache, iteration + 1, count % places);
        a + b
    } else {
        do_part_two_rec(cache, iteration + 1, count * 2024)
    };
    cache.insert(cache_key, retval);
    retval
}

fn width(x: u64) -> u32 {
    (x as f64 + 0.1).log10().ceil() as u32
}
