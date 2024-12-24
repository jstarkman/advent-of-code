use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

/// Remember to build with `--release`!
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let connections = parse_input(BufReader::new(f));
    let adj = make_graph_adj(connections.iter().copied(), 26 * 26);
    // Part one
    let p1 = do_part_one(&connections, &adj);
    dbg!(p1);
    // Part two
    let p2 = do_part_two(&adj);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> HashSet<(usize, usize)> {
    r.lines()
        .flat_map(|l| l.ok().into_iter())
        .map(|line| {
            let mut iter = line.split('-');
            let left = to_machine_id(iter.next().expect("sane input"));
            let right = to_machine_id(iter.next().expect("sane input"));
            // Connections are symmetric, so why worry about ordering?
            (left.min(right), left.max(right))
        })
        .collect()
}

/// Turns input IDs into `0..26^2` -range machine ID.
const fn to_machine_id(two_lowercase_ascii_letters: &str) -> usize {
    let b = two_lowercase_ascii_letters.as_bytes();
    (((b[0] - b'a') as usize) * 26) + ((b[1] - b'a') as usize)
}

const fn from_machine_id(id: usize) -> [u8; 2] {
    let car = (id / 26) as u8 + b'a';
    let cdr = (id % 26) as u8 + b'a';
    [car, cdr]
}

const START_T: usize = (b't' - b'a') as usize * 26;
const STOP_T_EXC: usize = START_T + 26;
const fn machine_id_starts_with_t(id: usize) -> bool {
    START_T <= id && id < STOP_T_EXC
}

fn make_graph_adj(edges: impl Iterator<Item = (usize, usize)>, n: usize) -> Vec<Vec<usize>> {
    let mut retval = vec![vec![]; n];
    for (a, b) in edges {
        retval[a].push(b); // forward
        retval[b].push(a); // backward
    }
    for to in retval.iter_mut() {
        to.sort_unstable();
    }
    retval
}

fn do_part_one(connections: &HashSet<(usize, usize)>, adj: &Vec<Vec<usize>>) -> u64 {
    let mut triangles = HashSet::new();
    for (from, to) in adj.iter().enumerate() {
        if to.is_empty() {
            continue;
        }
        // Hashes are slow and 26^3 is not very many.
        for (a, b) in connections.iter().copied() {
            // from always starts with `t`
            if !(machine_id_starts_with_t(from)
                || machine_id_starts_with_t(a)
                || machine_id_starts_with_t(b))
            {
                continue;
            }
            if adj[a].binary_search(&from).is_err() {
                continue;
            }
            if adj[b].binary_search(&from).is_err() {
                continue;
            }
            let mut tri = [from, a, b];
            tri.sort_unstable();
            triangles.insert(tri);
        }
    }
    triangles.len() as u64
}

/// Brute force; when built with `--release`, still takes under 100ms, including
/// the overhead of re-doing Part One and invoking Cargo.
fn do_part_two(adj: &Vec<Vec<usize>>) -> String {
    let mut fully_connected = HashSet::new();
    // The largest LAN must be contained within adj[this_one] = [and, some, of, these].
    // If it were split across multiple adj[i], then it would not be fully-connected.
    for (from1, to) in adj.iter().enumerate() {
        for to_me in 0..to.len() {
            let from2 = to[to_me];
            let mut maybe = HashSet::new();
            maybe.insert(from1);
            maybe.insert(from2);
            for from3 in to[to_me + 1..].iter().copied() {
                let still_fully_connected = maybe
                    .iter()
                    .all(|&id| adj[id].binary_search(&from3).is_ok());
                if still_fully_connected {
                    maybe.insert(from3);
                }
            }
            if maybe.len() > fully_connected.len() {
                fully_connected = maybe;
            }
        }
    }
    // LAN members -> password
    let mut buf: Vec<_> = fully_connected.into_iter().collect();
    buf.sort_unstable();
    let mut retval: String = buf
        .into_iter()
        // shame that .intersperse() is still limited to nightly
        .flat_map(|id| from_machine_id(id).into_iter().chain(std::iter::once(b',')))
        .map(|b| b as char)
        .collect();
    let _trailing_comma = retval.pop();
    retval
}
