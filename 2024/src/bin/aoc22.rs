use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

/// Remember to build with `--release`!
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let secrets = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(&secrets, 2000);
    dbg!(p1);
    // Part two
    let all_price_changes = get_all_price_changes(&secrets, 2000);
    // println!("{all_price_changes:?}");
    let p2 = do_part_two(&all_price_changes);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<u64> {
    r.lines()
        .flat_map(|l| l.ok().into_iter())
        .map(|line| line.parse().unwrap())
        .collect()
}

const fn mix_into(secret: u64, value: u64) -> u64 {
    secret ^ value
}

const fn prune(secret: u64) -> u64 {
    secret & 0x00FF_FFFF
}

const fn next_secret(secret: u64) -> u64 {
    let mut secret = secret;
    secret = mix_into(secret, secret << 6);
    secret = prune(secret);
    secret = mix_into(secret, secret >> 5);
    secret = prune(secret);
    secret = mix_into(secret, secret << 11);
    secret = prune(secret);
    secret
}

fn do_part_one(secrets: &[u64], iterations: usize) -> u64 {
    secrets
        .iter()
        .map(|secret| {
            let mut s = *secret;
            for _ in 0..iterations {
                s = next_secret(s);
            }
            s
        })
        // .inspect(|s| println!("{s}"))
        .sum()
}

fn get_all_price_changes(secrets: &[u64], iterations: usize) -> Vec<Vec<(u8, i8)>> {
    secrets
        .iter()
        .map(|secret| {
            let mut price_changes = Vec::with_capacity(iterations);
            let mut s = *secret;
            let mut previous_price = (*secret % 10) as i8;
            for _ in 0..iterations {
                s = next_secret(s);
                let price = (s % 10) as i8;
                let change = price - previous_price;
                price_changes.push((price as u8, change));
                previous_price = price;
            }
            price_changes
        })
        .collect()
}

fn do_part_two(all_price_changes: &Vec<Vec<(u8, i8)>>) -> u64 {
    let mut max_per_pattern: HashMap<u32, u64> = HashMap::new();
    for price_changes in all_price_changes.iter() {
        let mut seen_first: HashSet<u32> = HashSet::new();
        for pattern in price_changes.windows(4) {
            let price = pattern[3].0;
            let key = {
                let mut key = 0_u32;
                for (_price, change) in pattern.iter().copied() {
                    let change_bits = change.to_ne_bytes()[0] as u32;
                    key = (key << 8) | change_bits;
                }
                key
            };
            if !seen_first.insert(key) {
                continue;
            }
            *max_per_pattern.entry(key).or_default() += price as u64;
        }
    }
    max_per_pattern
        .values()
        .copied()
        .max()
        .expect("should have non-empty prices")
}
