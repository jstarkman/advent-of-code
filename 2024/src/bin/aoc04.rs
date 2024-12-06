use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let mut word_search = Vec::new();
    File::open(path_input)?.read_to_end(&mut word_search)?;
    let p1 = find_all_xmas(&word_search);
    dbg!(p1);
    // Part two
    let mut word_search = Vec::new();
    File::open(path_input)?.read_to_end(&mut word_search)?;
    let p2 = find_all_masxmas(&word_search);
    dbg!(p2);
    Ok(())
}

/// Poor man's sparse convolution kernels.
const OFFSETS: [[(usize, usize); 4]; 8] = [
    // Horizontally
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(3, 0), (2, 0), (1, 0), (0, 0)],
    // Vertically
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 3), (0, 2), (0, 1), (0, 0)],
    // NW-SE
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(3, 3), (2, 2), (1, 1), (0, 0)],
    // NE-SW
    [(0, 3), (1, 2), (2, 1), (3, 0)],
    [(3, 0), (2, 1), (1, 2), (0, 3)],
];
const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

fn find_all_xmas(word_search: &[u8]) -> u64 {
    // Newlines (always Unix-style) have not been stripped out.
    let width = word_search.iter().take_while(|&&b| b != b'\n').count();
    let height = word_search.iter().filter(|&&b| b == b'\n').count();
    // let kernel_size = XMAS.len();
    let mut found = 0;
    for y_ws in 0..height {
        for x_ws in 0..width {
            for offset in OFFSETS.iter() {
                let is_xmas = has_xmas(word_search, height, width, y_ws, x_ws, offset);
                if is_xmas {
                    // println!("Found ({x_ws}, {y_ws}) with {offset:?}");
                    found += 1;
                }
            }
        }
    }
    found
}

#[inline]
fn has_xmas(
    word_search: &[u8],
    height: usize,
    width: usize,
    y_ws: usize,
    x_ws: usize,
    offset: &[(usize, usize); 4],
) -> bool {
    let mut offset = offset.clone();
    for i in 0..offset.len() {
        offset[i].0 += x_ws;
        if offset[i].0 >= width {
            return false;
        }
        offset[i].1 += y_ws;
        if offset[i].1 >= height {
            return false;
        }
    }
    offset
        .into_iter()
        .map(|(x_r, y_r)| y_r * (width + 1) + x_r)
        .zip(XMAS.iter().cloned())
        .all(|(i_ws, b)| word_search[i_ws] == b)
}

/// Poor man's sparse convolution kernels.
const OFFSETS_MASXMAS: [[(usize, usize); 5]; 4] = [
    // Rotations are always MASMS order.
    [(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)], // left
    [(0, 0), (1, 1), (2, 2), (2, 0), (0, 2)], // top
    [(2, 2), (1, 1), (0, 0), (2, 0), (0, 2)], // right
    [(2, 2), (1, 1), (0, 0), (0, 2), (2, 0)], // right
];
const MASXMAS: [u8; 5] = [b'M', b'A', b'S', b'M', b'S'];

fn find_all_masxmas(word_search: &[u8]) -> u64 {
    // Newlines (always Unix-style) have not been stripped out.
    let width = word_search.iter().take_while(|&&b| b != b'\n').count();
    let height = word_search.iter().filter(|&&b| b == b'\n').count();
    // let kernel_size = XMAS.len();
    let mut found = 0;
    for y_ws in 0..height {
        for x_ws in 0..width {
            for offset in OFFSETS_MASXMAS.iter() {
                let is_xmas = has_masxmas(word_search, height, width, y_ws, x_ws, offset);
                if is_xmas {
                    // println!("Found ({x_ws}, {y_ws}) with {offset:?}");
                    found += 1;
                }
            }
        }
    }
    found
}

#[inline]
fn has_masxmas(
    word_search: &[u8],
    height: usize,
    width: usize,
    y_ws: usize,
    x_ws: usize,
    offset: &[(usize, usize); 5],
) -> bool {
    let mut offset = offset.clone();
    for i in 0..offset.len() {
        offset[i].0 += x_ws;
        if offset[i].0 >= width {
            return false;
        }
        offset[i].1 += y_ws;
        if offset[i].1 >= height {
            return false;
        }
    }
    offset
        .into_iter()
        .map(|(x_r, y_r)| y_r * (width + 1) + x_r)
        .zip(MASXMAS.iter().cloned())
        .all(|(i_ws, b)| word_search[i_ws] == b)
}
