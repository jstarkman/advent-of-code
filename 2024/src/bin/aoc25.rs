use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Remember to build with `--release`!
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let (keys, locks) = parse_input(BufReader::new(f));
    // Have 250 of each.
    // println!("{keys:?}");
    // println!("{locks:?}");
    // Part one
    let p1 = do_part_one(&keys, &locks, 5);
    dbg!(p1);
    // Part two
    // let p2 = do_part_two();
    // dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> (Vec<Profile>, Vec<Profile>) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut profile = Profile::default();
    let mut is_lock = None;
    for line in r
        .lines()
        .flat_map(|l| l.ok().into_iter())
        .chain(["".into()])
    {
        if line.is_empty() {
            profile.0.iter_mut().for_each(|h| *h -= 1);
            if is_lock.unwrap() {
                locks.push(profile);
            } else {
                keys.push(profile);
            }
            profile = Profile::default();
            is_lock = None;
            continue;
        }
        if is_lock.is_none() {
            is_lock = Some(line.chars().next().unwrap() == '#');
        }
        for (i, ch) in line.char_indices() {
            let mass = (ch == '#') as i32;
            profile.0[i] += mass;
        }
    }
    (keys, locks)
}

#[derive(Clone, Copy, Debug, Default)]
struct Profile([i32; 5]);

fn do_part_one(keys: &[Profile], locks: &[Profile], max_height: i32) -> u64 {
    // `250 * 249 / 2` is small, so brute force works.
    let mut could_fit = 0;
    for key in keys {
        for lock in locks {
            let overlaps = key
                .0
                .iter()
                .zip(lock.0.iter())
                .map(|(k, l)| k + l)
                .any(|h| h > max_height);
            if !overlaps {
                could_fit += 1;
            }
        }
    }
    could_fit
}
