use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let (have, want) = parse_input(BufReader::new(f));
    // println!("{have:?}");
    let p1 = do_part_one(&have, &want);
    dbg!(p1);
    // Part two
    let p2 = do_part_two(&have, &want);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> (TrieNode5, Vec<String>) {
    let mut iter_lines = r.lines().flat_map(|l| l.ok().into_iter());
    let mut have = TrieNode5::new();
    iter_lines
        .next()
        .expect("sane input")
        .split(", ")
        .for_each(|t| {
            have.insert(t);
        });
    let _blank = iter_lines.next();
    let want = iter_lines.collect();
    (have, want)
}

fn do_part_one(have: &TrieNode5, want: &[String]) -> usize {
    let mut can = 0;
    for wanted in want.iter() {
        if is_possible(have, wanted) {
            can += 1;
        }
    }
    can
}

fn is_possible(have: &TrieNode5, wanted: &str) -> bool {
    if wanted.is_empty() {
        return true;
    }
    // println!("Looking for {wanted}");
    for i in 1..=wanted.len() {
        let prefix = &wanted[..i];
        // println!("\t0 .. {i} -> {prefix}");
        if have.contains(prefix) {
            if is_possible(have, &wanted[i..]) {
                return true;
            }
        }
    }
    // println!("\tFailed to find {wanted}");
    false
}

fn do_part_two(have: &TrieNode5, want: &[String]) -> u64 {
    let mut retval = 0;
    let mut cache = HashMap::new();
    for wanted in want.iter() {
        let me = count_the_ways(&mut cache, have, wanted);
        retval += me;
    }
    retval
}

fn count_the_ways(cache: &mut HashMap<String, u64>, have: &TrieNode5, wanted: &str) -> u64 {
    if wanted.is_empty() {
        return 1;
    }
    if let Some(&me) = cache.get(wanted) {
        return me;
    }
    let mut retval = 0;
    for i in 1..=wanted.len() {
        let prefix = &wanted[..i];
        if have.contains(prefix) {
            let me = count_the_ways(cache, have, &wanted[i..]);
            retval += me;
        }
    }
    cache.insert(wanted.to_owned(), retval);
    retval
}

struct TrieNode5Index(usize);

impl TryFrom<char> for TrieNode5Index {
    type Error = AOCParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        // MTG order
        match value {
            'w' => Ok(TrieNode5Index(0)),
            'u' => Ok(TrieNode5Index(1)),
            'b' => Ok(TrieNode5Index(2)),
            'r' => Ok(TrieNode5Index(3)),
            'g' => Ok(TrieNode5Index(4)),
            _ => Err(AOCParseError),
        }
    }
}

impl Deref for TrieNode5Index {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
struct TrieNode5 {
    children: [Option<Box<TrieNode5>>; 5],
    word_ends_here: bool,
}

impl TrieNode5 {
    fn new() -> TrieNode5 {
        Self {
            children: Default::default(),
            word_ends_here: false,
        }
    }

    fn contains(&self, word: &str) -> bool {
        if let Some(c) = word.chars().next() {
            let w: TrieNode5Index = c.try_into().expect("sane");
            if let Some(ord) = &self.children[*w] {
                ord.contains(&word[1..])
            } else {
                false
            }
        } else {
            self.word_ends_here
        }
    }

    fn insert(&mut self, word: &str) -> () {
        if let Some(c) = word.chars().next() {
            let w: TrieNode5Index = c.try_into().expect("sane");
            self.children[*w]
                .get_or_insert_with(|| Box::new(TrieNode5::new()))
                .insert(&word[1..]);
        } else {
            self.word_ends_here = true;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AOCParseError;
