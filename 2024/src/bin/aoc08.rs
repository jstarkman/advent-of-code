use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const ANTINODE_MARKER: u8 = b'#';
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let map = parse_input(BufReader::new(f));
    // Part one
    let antinode_map = find_antinodes(&map, ANTINODE_MARKER);
    // for row in antinode_map.iter() {
    //     println!("{}", String::from_utf8_lossy(row));
    // }
    let p1: u64 = antinode_map
        .iter()
        .map(|row| row.iter().filter(|&&c| c == ANTINODE_MARKER).count() as u64)
        .sum();
    dbg!(p1);
    // Part two
    let resonant_antinode_map = find_resonant_antinodes(&map, ANTINODE_MARKER);
    let p2: u64 = resonant_antinode_map
        .iter()
        .map(|row| row.iter().filter(|&&c| c == ANTINODE_MARKER).count() as u64)
        .sum();
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Vec<Vec<u8>> {
    r.lines().map(|l| l.expect("sanity").into_bytes()).collect()
}

fn find_antinodes(map: &[Vec<u8>], marker: u8) -> Vec<Vec<u8>> {
    let height = map.len();
    let width = map[0].len();
    let mut antinode_map = vec![vec![b'.'; width]; height];
    let nodes_per_frequency = find_nodes(map);
    // dbg!(&nodes_per_frequency);
    for (_freq, coords) in nodes_per_frequency.iter() {
        for i in 0..coords.len() {
            // Need both directions, so not `i+1 .. coords.len()`
            for ii in 0..coords.len() {
                if i == ii {
                    continue;
                }
                let (x0, y0) = coords[i];
                let (x1, y1) = coords[ii];
                let (dx, dy) = (x1.wrapping_sub(x0), y1.wrapping_sub(y0));
                let (xx, yy) = (x1.wrapping_add(dx), y1.wrapping_add(dy));
                if let Some(row) = antinode_map.get_mut(yy) {
                    if let Some(cell) = row.get_mut(xx) {
                        *cell = marker;
                    }
                }
            }
        }
    }
    antinode_map
}

fn find_resonant_antinodes(map: &[Vec<u8>], marker: u8) -> Vec<Vec<u8>> {
    let height = map.len();
    let width = map[0].len();
    let mut antinode_map = vec![vec![b'.'; width]; height];
    let nodes_per_frequency = find_nodes(map);
    // dbg!(&nodes_per_frequency);
    for (_freq, coords) in nodes_per_frequency.iter() {
        // All towers are also resonant nodes.
        for &(x, y) in coords.iter() {
            antinode_map[y][x] = marker;
        }
        for i in 0..coords.len() {
            // Need both directions, so not `i+1 .. coords.len()`
            for ii in 0..coords.len() {
                if i == ii {
                    continue;
                }
                let (x0, y0) = coords[i];
                let (x1, y1) = coords[ii];
                let (dx, dy) = (x1.wrapping_sub(x0), y1.wrapping_sub(y0));
                let (mut xx, mut yy) = (x1, y1);
                loop {
                    xx = xx.wrapping_add(dx);
                    yy = yy.wrapping_add(dy);
                    if let Some(row) = antinode_map.get_mut(yy) {
                        if let Some(cell) = row.get_mut(xx) {
                            *cell = marker;
                            continue;
                        }
                    }
                    break;
                }
            }
        }
    }
    antinode_map
}

fn find_nodes(map: &[Vec<u8>]) -> HashMap<u8, Vec<(usize, usize)>> {
    let mut nodes_per_frequency: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell.is_ascii_alphanumeric() {
                nodes_per_frequency.entry(cell).or_default().push((x, y));
            }
        }
    }
    nodes_per_frequency
}
