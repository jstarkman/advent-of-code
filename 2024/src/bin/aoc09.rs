use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufReader, Read},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let disk_map = parse_input(BufReader::new(f));
    // println!("{:?}", &disk_map[0..100.min(disk_map.len())]);
    // println!("{:?}", &disk_map);
    // Part one
    let mut disk_map_p1 = disk_map.clone();
    compact_fragmented(&mut disk_map_p1);
    let p1 = checksum(&disk_map_p1);
    dbg!(p1);
    // Part two
    let mut disk_map_p2 = disk_map; // last use so no .clone()
    compact_nofrag(&mut disk_map_p2);
    // println!("{:?}", &disk_map_p2[0..100.min(disk_map.len())]);
    // println!("{:?}", &disk_map_p2);
    let p2 = checksum(&disk_map_p2);
    dbg!(p2);
    Ok(())
}

const NO_FILE_ID: u16 = u16::MAX;

fn parse_input(mut r: BufReader<File>) -> Vec<u16> {
    let mut buf = vec![];
    r.read_to_end(&mut buf).expect("sane");
    assert!(!buf.is_empty(), "need an input");
    while buf.last().unwrap().is_ascii_whitespace() {
        let _probably_a_newline = buf.pop();
    }
    // The full-size input has 20k bytes, so 10k pairs of at most (9+9) blocks.
    // 180k blocks is not a large hard drive.
    let mut disk_map = vec![];
    let mut file_id = 0_u16;
    for ch in buf.chunks(2) {
        let (files, free_space) = (ch[0], ch.get(1).cloned().unwrap_or(b'0'));
        if files == b'0' {
            continue; // these count towards file IDs
        }
        for _ in b'0'..files {
            disk_map.push(file_id);
        }
        for _ in b'0'..free_space {
            disk_map.push(NO_FILE_ID);
        }
        file_id += 1;
    }
    disk_map
}

fn compact_fragmented(disk_map: &mut [u16]) {
    let mut lo = 0;
    let mut hi = disk_map.len() - 1;
    while lo < hi {
        while let Some(&b) = disk_map.get(lo) {
            if b != NO_FILE_ID {
                lo += 1;
            } else {
                break;
            }
        }
        if lo >= hi {
            break;
        }
        // Now, disk_map[lo] must exist and be empty.
        while let Some(&b) = disk_map.get(hi) {
            if b == NO_FILE_ID {
                if hi == 0 {
                    break;
                }
                hi -= 1;
            } else {
                break;
            }
        }
        if lo >= hi {
            break;
        }
        // Now, disk_map[hi] must exist and be non-empty.
        disk_map.swap(lo, hi);
        lo += 1;
        hi -= 1;
    }
}

fn compact_nofrag(disk_map: &mut [u16]) {
    let n = disk_map.len();
    let mut freelist = scan_for_freelist(disk_map);
    let widest = *freelist.keys().max().unwrap();
    let mut i = n - 1;
    let mut last_file_id = u16::MAX;
    loop {
        let file_id = disk_map[i];
        if file_id == NO_FILE_ID {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }
        if file_id >= last_file_id {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }
        let mut i_file = i;
        while disk_map[i_file] == file_id {
            if i_file == 0 {
                break; // already at beginning; how to move left?
            }
            i_file -= 1;
        }
        if i_file == 0 {
            break; // ibid
        }
        let width = i - i_file;
        // dbg!(i, file_id, width, &freelist);
        // Left-most, not narrowest
        let leftmost_opening = (width..=widest)
            .filter_map(|wider| {
                let Reverse(i_free) = freelist.get(&wider).and_then(|v| v.peek().cloned())?;
                if i_file <= i_free {
                    // oops; only want to move left
                    return None;
                }
                Some((i_free, wider))
            })
            .min();
        if let Some((i_free, wider)) = leftmost_opening {
            let _same_as_i_free = freelist.get_mut(&wider).expect("worked above").pop();
            // Move the file
            disk_map.copy_within(i_file + 1..=i, i_free);
            for idx in i_file + 1..=i {
                disk_map[idx] = NO_FILE_ID; // for debugging; never read
            }
            // Did we split a block?
            let leftover = wider - width;
            if leftover > 0 {
                let new_i_free = i_free + width;
                freelist
                    .entry(leftover)
                    .or_default()
                    .push(Reverse(new_i_free));
            }
            // Although we left a hole where we were, that hole will always
            // be to the right of `i`, so that hole will never be useful for
            // `freelist` despite belonging there.
        }
        i = i_file;
        last_file_id = file_id;
    }
}

fn scan_for_freelist(disk_map: &mut [u16]) -> HashMap<usize, BinaryHeap<Reverse<usize>>> {
    let n = disk_map.len();
    let mut freelist: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();
    let mut i = 0;
    // Loop invariant: always start on a real file ID
    assert_ne!(NO_FILE_ID, disk_map[0]);
    while i < n {
        while i < n && disk_map[i] != NO_FILE_ID {
            i += 1;
        }
        if i >= n {
            break;
        }
        let mut ii = i;
        while ii < n && disk_map[ii] == NO_FILE_ID {
            ii += 1;
        }
        let width = ii - i;
        freelist.entry(width).or_default().push(Reverse(i));
        i = ii;
    }
    freelist
}

fn checksum(disk_map: &[u16]) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, &file_id)| (file_id != NO_FILE_ID).then(|| (i as u64) * (file_id as u64)))
        .sum()
}
