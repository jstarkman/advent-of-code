use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let mut lists = parse_to_two_lists(BufReader::new(f));
    lists[0].sort_unstable();
    lists[1].sort_unstable();
    let (left, right) = lists.split_at(1);
    let total_distance = total_distance(&left[0], &right[0]);
    dbg!(total_distance);
    let similarity_score = similarity_score(&left[0], &right[0]);
    dbg!(similarity_score);
    Ok(())
}

fn parse_to_two_lists(r: BufReader<File>) -> [Vec<i32>; 2] {
    let mut lists = [vec![], vec![]];
    for line in r.lines() {
        let line = line.expect("Input should be a sane Unix text file.");
        line.split_ascii_whitespace()
            .filter(|w| !w.is_empty())
            .zip(lists.iter_mut())
            .for_each(|(w, l)| {
                let id = w.parse::<i32>().expect("sane input");
                l.push(id);
            });
    }
    lists
}

fn total_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn similarity_score(left: &[i32], right: &[i32]) -> i64 {
    let mut freq: HashMap<&i32, i32> = HashMap::new();
    for id in right.iter() {
        *freq.entry(id).or_default() += 1;
    }
    left.iter()
        .map(|id| freq.get(id).unwrap_or(&0) * id)
        .map(|sim| sim as i64)
        .sum()
}
