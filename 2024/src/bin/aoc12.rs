use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    let f = File::open(path_input)?;
    let map = parse_input(BufReader::new(f));
    // Part one
    let (p1, p2) = map.do_both_parts();
    dbg!(p1);
    dbg!(p2);
    Ok(())
}

fn parse_input(r: BufReader<File>) -> Map {
    let mut buf = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in r.lines() {
        let line = line.expect("sane");
        width = line.len(); // both inputs are of constant width
        for b in line.bytes() {
            buf.push(b);
        }
        height += 1;
    }
    Map {
        tiles: buf,
        width,
        height,
    }
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn do_both_parts(&self) -> (u64, u64) {
        let mut seen = vec![false; self.tiles.len()];
        let mut map_ids = vec![0_16; self.tiles.len()];
        let mut ap_ids = vec![];
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !seen[i] {
                    // let t = self.tiles[i];
                    let mut ap = AreaPerimeter(0, 0);
                    let region_id = ap_ids.len() as u16;
                    self.enclose_region(&mut seen, &mut ap, (x, y), &mut map_ids, region_id);
                    // println!("{} -> {ap:?}", char::from_u32(t as u32).unwrap());
                    ap_ids.push(ap);
                }
                i += 1;
            }
        }
        let price_p1: u64 = ap_ids.iter().map(|ap| (ap.0 * ap.1) as u64).sum();
        self.find_straights(|x, y| {
            let region_id = map_ids[y * self.width + x];
            // println!("\t{x} {y}\t{region_id}");
            ap_ids[region_id as usize].1 -= 1;
        });
        let price_p2: u64 = ap_ids
            .iter()
            // .inspect(|ap| {
            //     dbg!(ap);
            // })
            .map(|ap| (ap.0 * ap.1) as u64)
            .sum();
        (price_p1, price_p2)
    }

    fn enclose_region(
        &self,
        seen: &mut [bool],
        ap: &mut AreaPerimeter,
        (x, y): (usize, usize),
        map_ids: &mut [u16],
        region_id: u16,
    ) {
        let idx = y * self.width + x;
        if seen[idx] {
            return;
        }
        seen[idx] = true;
        map_ids[idx] = region_id;
        ap.0 += 1;
        let here = self.tiles[idx];
        let mut perimeter_potential = 4;
        for (xx, yy) in NeighborIterator::new(self.height, self.width, x, y, false) {
            let there = self.tiles[yy * self.width + xx];
            if here == there {
                self.enclose_region(seen, ap, (xx, yy), map_ids, region_id);
                perimeter_potential -= 1;
            }
        }
        // println!(
        //     "{}\t+p {perimeter_potential}",
        //     char::from_u32(here as u32).unwrap()
        // );
        ap.1 += perimeter_potential;
    }

    /// `f(x,y)` will be called once for each half.
    fn find_straights<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        // Edges, top
        for x in 0..self.width - 1 {
            if self.tiles[x] == self.tiles[x + 1] {
                f(x, 0);
            }
        }
        // Edges, bottom
        let last_row = (self.height - 1) * self.width;
        for x in 0..self.width - 1 {
            let xx = x + last_row;
            if self.tiles[xx] == self.tiles[xx + 1] {
                f(x, self.height - 1);
            }
        }
        // Edges, left/right
        let last_col = self.width - 1;
        for y in 0..self.height - 1 {
            let h = y * self.width;
            if self.tiles[h + 0] == self.tiles[(h + self.width) + 0] {
                f(0, y);
            }
            if self.tiles[h + last_col] == self.tiles[(h + self.width) + last_col] {
                f(last_col, y);
            }
        }
        // Middle
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                // Poor man's up/down or left/right 2D convolution kernel.
                let ul = y * self.width + x;
                let ur = ul + 1;
                let ll = ul + self.width;
                let lr = ll + 1;
                // up/down
                if self.tiles[ul] != self.tiles[ll] && self.tiles[ur] != self.tiles[lr] {
                    if self.tiles[ul] == self.tiles[ur] {
                        f(x, y);
                    }
                    if self.tiles[ll] == self.tiles[lr] {
                        f(x, y + 1);
                    }
                }
                // left/right
                if self.tiles[ul] != self.tiles[ur] && self.tiles[ll] != self.tiles[lr] {
                    if self.tiles[ul] == self.tiles[ll] {
                        f(x, y);
                    }
                    if self.tiles[ur] == self.tiles[lr] {
                        f(x + 1, y);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct AreaPerimeter(usize, usize);

//region Iterator over neighboring tiles in a grid; four- or eight-connected.
struct NeighborIterator {
    height: isize,
    width: isize,
    x: isize,
    y: isize,
    scan_index: usize,
    connect: usize,
}

impl NeighborIterator {
    fn new(
        height: usize,
        width: usize,
        x: usize,
        y: usize,
        is_eight_connected: bool,
    ) -> NeighborIterator {
        Self {
            height: height as isize,
            width: width as isize,
            x: x as isize,
            y: y as isize,
            scan_index: 0,
            connect: 4 * (is_eight_connected as usize + 1),
        }
    }
    const OFFSETS4: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    #[rustfmt::skip]
    const OFFSETS8: [(isize, isize); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0), /*0, 0*/ (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
}

impl Iterator for NeighborIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.scan_index < self.connect {
            let (dx, dy) = match self.connect {
                4 => NeighborIterator::OFFSETS4[self.scan_index].clone(),
                8 => NeighborIterator::OFFSETS8[self.scan_index].clone(),
                _ => unreachable!(),
            };
            self.scan_index += 1;
            let c = (self.x + dx, self.y + dy);
            let can_dx = 0 <= c.0 && c.0 < self.width;
            let can_dy = 0 <= c.1 && c.1 < self.height;
            if can_dx && can_dy {
                return Some((c.0 as usize, c.1 as usize));
            }
        }
        None
    }
}
//endregion
