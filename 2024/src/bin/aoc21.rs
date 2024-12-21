use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::LazyLock,
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let final_codes = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(&final_codes, 3);
    dbg!(p1);
    // Part two
    let p2 = do_part_one(&final_codes, 26);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<String> {
    r.lines().flat_map(|l| l.ok().into_iter()).collect()
}

fn do_part_one(goals: &[String], layers: usize) -> usize {
    let mut cache = HashMap::new();
    let mut complexities = 0;
    for goal in goals.iter() {
        let len = shortest_sequence(&mut cache, goal, layers);
        let numeric_part = goal[0..goal.len() - 1].parse::<usize>().unwrap();
        let complexity = len * numeric_part;
        complexities += complexity;
    }
    complexities
}

fn shortest_sequence<'goal>(
    cache: &mut HashMap<(&'goal str, usize), usize>,
    goal: &'goal str,
    layer: usize,
) -> usize {
    if layer == 0 {
        return goal.len();
    }
    let cache_key = (goal, layer);
    if let Some(&len) = cache.get(&cache_key) {
        return len;
    }
    // always starts from (and returns to) button `A` on each keypad.
    let mut len = 0;
    let from_a = [b'A', goal.as_bytes()[0]];
    for w in std::iter::once(&from_a[..]).chain(goal.as_bytes().windows(2)) {
        let from_to = (w[0], w[1]);
        let subseq = STATE_TRANSITION_TABLE[&from_to].as_str();
        let rec = shortest_sequence(cache, subseq, layer - 1);
        len += rec;
    }
    cache.insert(cache_key, len);
    len
}

/// (from, to) -> "path here"
static STATE_TRANSITION_TABLE: LazyLock<HashMap<(u8, u8), String>> =
    LazyLock::new(make_state_transition_table);
fn make_state_transition_table() -> HashMap<(u8, u8), String> {
    let mut retval = HashMap::new();
    retval.insert((b'^', b'^'), "A".to_owned());
    retval.insert((b'^', b'<'), "v<A".to_owned());
    retval.insert((b'^', b'>'), "v>A".to_owned());
    retval.insert((b'^', b'A'), ">A".to_owned());
    retval.insert((b'^', b'v'), "vA".to_owned());
    retval.insert((b'<', b'^'), ">^A".to_owned());
    retval.insert((b'<', b'<'), "A".to_owned());
    retval.insert((b'<', b'>'), ">>A".to_owned());
    retval.insert((b'<', b'A'), ">>^A".to_owned());
    retval.insert((b'<', b'v'), ">A".to_owned());
    retval.insert((b'>', b'^'), "<^A".to_owned());
    retval.insert((b'>', b'<'), "<<A".to_owned());
    retval.insert((b'>', b'>'), "A".to_owned());
    retval.insert((b'>', b'A'), "^A".to_owned());
    retval.insert((b'>', b'v'), "<A".to_owned());
    retval.insert((b'0', b'0'), "A".to_owned());
    retval.insert((b'0', b'1'), "^<A".to_owned());
    retval.insert((b'0', b'2'), "^A".to_owned());
    retval.insert((b'0', b'3'), "^>A".to_owned());
    retval.insert((b'0', b'4'), "^<^A".to_owned());
    retval.insert((b'0', b'5'), "^^A".to_owned());
    retval.insert((b'0', b'6'), "^^>A".to_owned());
    retval.insert((b'0', b'7'), "^^^<A".to_owned());
    retval.insert((b'0', b'8'), "^^^A".to_owned());
    retval.insert((b'0', b'9'), "^^^>A".to_owned());
    retval.insert((b'0', b'A'), ">A".to_owned());
    retval.insert((b'1', b'0'), ">vA".to_owned());
    retval.insert((b'1', b'1'), "A".to_owned());
    retval.insert((b'1', b'2'), ">A".to_owned());
    retval.insert((b'1', b'3'), ">>A".to_owned());
    retval.insert((b'1', b'4'), "^A".to_owned());
    retval.insert((b'1', b'5'), "^>A".to_owned());
    retval.insert((b'1', b'6'), "^>>A".to_owned());
    retval.insert((b'1', b'7'), "^^A".to_owned());
    retval.insert((b'1', b'8'), "^^>A".to_owned());
    retval.insert((b'1', b'9'), "^^>>A".to_owned());
    retval.insert((b'1', b'A'), ">>vA".to_owned());
    retval.insert((b'2', b'0'), "vA".to_owned());
    retval.insert((b'2', b'1'), "<A".to_owned());
    retval.insert((b'2', b'2'), "A".to_owned());
    retval.insert((b'2', b'3'), ">A".to_owned());
    retval.insert((b'2', b'4'), "<^A".to_owned());
    retval.insert((b'2', b'5'), "^A".to_owned());
    retval.insert((b'2', b'6'), "^>A".to_owned());
    retval.insert((b'2', b'7'), "<^^A".to_owned());
    retval.insert((b'2', b'8'), "^^A".to_owned());
    retval.insert((b'2', b'9'), "^^>A".to_owned());
    retval.insert((b'2', b'A'), "v>A".to_owned());
    retval.insert((b'3', b'0'), "<vA".to_owned());
    retval.insert((b'3', b'1'), "<<A".to_owned());
    retval.insert((b'3', b'2'), "<A".to_owned());
    retval.insert((b'3', b'3'), "A".to_owned());
    retval.insert((b'3', b'4'), "<<^A".to_owned());
    retval.insert((b'3', b'5'), "<^A".to_owned());
    retval.insert((b'3', b'6'), "^A".to_owned());
    retval.insert((b'3', b'7'), "<<^^A".to_owned());
    retval.insert((b'3', b'8'), "<^^A".to_owned());
    retval.insert((b'3', b'9'), "^^A".to_owned());
    retval.insert((b'3', b'A'), "vA".to_owned());
    retval.insert((b'4', b'0'), ">vvA".to_owned());
    retval.insert((b'4', b'1'), "vA".to_owned());
    retval.insert((b'4', b'2'), "v>A".to_owned());
    retval.insert((b'4', b'3'), "v>>A".to_owned());
    retval.insert((b'4', b'4'), "A".to_owned());
    retval.insert((b'4', b'5'), ">A".to_owned());
    retval.insert((b'4', b'6'), ">>A".to_owned());
    retval.insert((b'4', b'7'), "^A".to_owned());
    retval.insert((b'4', b'8'), "^>A".to_owned());
    retval.insert((b'4', b'9'), "^>>A".to_owned());
    retval.insert((b'4', b'A'), ">>vvA".to_owned());
    retval.insert((b'5', b'0'), "vvA".to_owned());
    retval.insert((b'5', b'1'), "<vA".to_owned());
    retval.insert((b'5', b'2'), "vA".to_owned());
    retval.insert((b'5', b'3'), "v>A".to_owned());
    retval.insert((b'5', b'4'), "<A".to_owned());
    retval.insert((b'5', b'5'), "A".to_owned());
    retval.insert((b'5', b'6'), ">A".to_owned());
    retval.insert((b'5', b'7'), "<^A".to_owned());
    retval.insert((b'5', b'8'), "^A".to_owned());
    retval.insert((b'5', b'9'), "^>A".to_owned());
    retval.insert((b'5', b'A'), "vv>A".to_owned());
    retval.insert((b'6', b'0'), "<vvA".to_owned());
    retval.insert((b'6', b'1'), "<<vA".to_owned());
    retval.insert((b'6', b'2'), "<vA".to_owned());
    retval.insert((b'6', b'3'), "vA".to_owned());
    retval.insert((b'6', b'4'), "<<A".to_owned());
    retval.insert((b'6', b'5'), "<A".to_owned());
    retval.insert((b'6', b'6'), "A".to_owned());
    retval.insert((b'6', b'7'), "<<^A".to_owned());
    retval.insert((b'6', b'8'), "<^A".to_owned());
    retval.insert((b'6', b'9'), "^A".to_owned());
    retval.insert((b'6', b'A'), "vvA".to_owned());
    retval.insert((b'7', b'0'), ">vvvA".to_owned());
    retval.insert((b'7', b'1'), "vvA".to_owned());
    retval.insert((b'7', b'2'), "vv>A".to_owned());
    retval.insert((b'7', b'3'), "vv>>A".to_owned());
    retval.insert((b'7', b'4'), "vA".to_owned());
    retval.insert((b'7', b'5'), "v>A".to_owned());
    retval.insert((b'7', b'6'), "v>>A".to_owned());
    retval.insert((b'7', b'7'), "A".to_owned());
    retval.insert((b'7', b'8'), ">A".to_owned());
    retval.insert((b'7', b'9'), ">>A".to_owned());
    retval.insert((b'7', b'A'), ">>vvvA".to_owned());
    retval.insert((b'8', b'0'), "vvvA".to_owned());
    retval.insert((b'8', b'1'), "<vvA".to_owned());
    retval.insert((b'8', b'2'), "vvA".to_owned());
    retval.insert((b'8', b'3'), "vv>A".to_owned());
    retval.insert((b'8', b'4'), "<vA".to_owned());
    retval.insert((b'8', b'5'), "vA".to_owned());
    retval.insert((b'8', b'6'), "v>A".to_owned());
    retval.insert((b'8', b'7'), "<A".to_owned());
    retval.insert((b'8', b'8'), "A".to_owned());
    retval.insert((b'8', b'9'), ">A".to_owned());
    retval.insert((b'8', b'A'), "vvv>A".to_owned());
    retval.insert((b'9', b'0'), "<vvvA".to_owned());
    retval.insert((b'9', b'1'), "<<vvA".to_owned());
    retval.insert((b'9', b'2'), "<vvA".to_owned());
    retval.insert((b'9', b'3'), "vvA".to_owned());
    retval.insert((b'9', b'4'), "<<vA".to_owned());
    retval.insert((b'9', b'5'), "<vA".to_owned());
    retval.insert((b'9', b'6'), "vA".to_owned());
    retval.insert((b'9', b'7'), "<<A".to_owned());
    retval.insert((b'9', b'8'), "<A".to_owned());
    retval.insert((b'9', b'9'), "A".to_owned());
    retval.insert((b'9', b'A'), "vvvA".to_owned());
    retval.insert((b'A', b'^'), "<A".to_owned());
    retval.insert((b'A', b'<'), "v<<A".to_owned());
    retval.insert((b'A', b'>'), "vA".to_owned());
    retval.insert((b'A', b'0'), "<A".to_owned());
    retval.insert((b'A', b'1'), "^<<A".to_owned());
    retval.insert((b'A', b'2'), "<^A".to_owned());
    retval.insert((b'A', b'3'), "^A".to_owned());
    retval.insert((b'A', b'4'), "^^<<A".to_owned());
    retval.insert((b'A', b'5'), "<^^A".to_owned());
    retval.insert((b'A', b'6'), "^^A".to_owned());
    retval.insert((b'A', b'7'), "^^^<<A".to_owned());
    retval.insert((b'A', b'8'), "<^^^A".to_owned());
    retval.insert((b'A', b'9'), "^^^A".to_owned());
    retval.insert((b'A', b'A'), "A".to_owned());
    retval.insert((b'A', b'v'), "<vA".to_owned());
    retval.insert((b'v', b'^'), "^A".to_owned());
    retval.insert((b'v', b'<'), "<A".to_owned());
    retval.insert((b'v', b'>'), ">A".to_owned());
    retval.insert((b'v', b'A'), "^>A".to_owned());
    retval.insert((b'v', b'v'), "A".to_owned());
    retval
}
