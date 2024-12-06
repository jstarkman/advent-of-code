use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let (orderings, mut updates) = parse_input(BufReader::new(f));
    // Part one
    let mut p1 = 0;
    updates.retain(|update| {
        let is_valid = check_valid_brute_force(update, &orderings);
        if is_valid {
            // "the Elves also need to know the *middle page number* of each update"
            let i = update.0.len() / 2;
            p1 += update.0[i] as u64;
        }
        !is_valid
    });
    dbg!(p1);
    let mut p2 = 0;
    // Part two, where `updates` are all invalid.
    let adj = to_adjacency_list(&orderings);
    // println!("adj {adj:?}");
    for update in updates.iter_mut() {
        let old_len = update.0.len();
        shuffle_valid(&adj, update);
        assert!(old_len == update.0.len(), "shuffles are not destructive");
        let is_valid = check_valid_brute_force(update, &orderings);
        assert!(is_valid, "Then why did we shuffle it?");
        let i = update.0.len() / 2;
        p2 += update.0[i] as u64;
    }
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> (Vec<PageOrdering>, Vec<PageUpdate>) {
    let mut orderings = vec![];
    // Separator for each data type.
    let mut have_reached_updates = false;
    let mut updates = vec![];
    for line in r.lines() {
        let line = line.expect("sane input");
        if line.is_empty() {
            have_reached_updates = true;
            continue;
        }
        if have_reached_updates {
            let update: Vec<_> = line
                .split(',')
                .map(|s| s.parse::<u8>().expect("sane input"))
                .collect();
            assert!(
                update.len() % 2 == 1,
                "updates must have an odd number of pages"
            );
            updates.push(PageUpdate(update));
        } else {
            // All page values in in 11 ..= 99
            let earlier = line
                .get(0..2)
                .expect("sane input")
                .parse::<u8>()
                .expect("sane input");
            let later = line
                .get(3..5)
                .expect("sane input")
                .parse::<u8>()
                .expect("sane input");
            assert!(earlier != later, "absurd input");
            orderings.push(PageOrdering { earlier, later });
        }
    }
    (orderings, updates)
}

fn check_valid_brute_force(update: &PageUpdate, orderings: &[PageOrdering]) -> bool {
    for &PageOrdering { earlier, later } in orderings.iter() {
        let mut iter_up = update.0.iter();
        let mut seen_later = false;
        while let Some(&next) = iter_up.next() {
            if next == later {
                seen_later = true;
            } else if next == earlier {
                if seen_later == true {
                    return false;
                }
            }
        }
    }
    true
}

fn to_adjacency_list(orderings: &[PageOrdering]) -> HashMap<u8, Vec<u8>> {
    let mut adj: HashMap<u8, Vec<u8>> = HashMap::new();
    for &PageOrdering { earlier, later } in orderings.iter() {
        adj.entry(earlier).or_default().push(later);
        adj.entry(later).or_default();
    }
    adj
}

fn shuffle_valid(adj: &HashMap<u8, Vec<u8>>, update: &mut PageUpdate) {
    // println!("update {update:?}");
    update.0.sort_by(|a, b| {
        if adj[a].contains(b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    // println!("\t-> {update:?}");
}

#[derive(Debug)]
struct PageOrdering {
    earlier: u8,
    later: u8,
}

#[derive(Debug)]
struct PageUpdate(Vec<u8>);
