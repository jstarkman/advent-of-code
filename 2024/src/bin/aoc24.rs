use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// Remember to build with `--release`!
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path_input = args.get(1).expect("Should have an input file");
    // Part one
    let f = File::open(path_input)?;
    let (inputs, gates) = parse_input(BufReader::new(f));
    // Part one
    let p1 = do_part_one(inputs, &gates);
    println!("p1 = {p1:#010b}");
    dbg!(p1);
    // Part two; brute force was too slow, so did it visually with graphviz instead.
    // let p2 = do_part_two(gates);
    // dbg!(p2);
    let gv = generate_graphviz(&gates, 46);
    println!("{gv}");
    Ok(())
}

fn parse_input(r: BufReader<File>) -> (HashMap<String, bool>, Vec<[String; 4]>) {
    let mut inputs = HashMap::new();
    let mut gates = vec![];
    let mut have_reached_gates = false;
    for line in r.lines().flat_map(|l| l.ok().into_iter()) {
        if line.is_empty() {
            have_reached_gates = true;
            continue;
        }
        if have_reached_gates {
            let mut iter_parts = line.split(" ");
            let id0 = iter_parts.next().expect("sane input").to_owned();
            let op = iter_parts.next().expect("sane input").to_owned();
            let id1 = iter_parts.next().expect("sane input").to_owned();
            let _arrow = iter_parts.next().expect("sane input").to_owned();
            let id2 = iter_parts.next().expect("sane input").to_owned();
            gates.push([id0, op, id1, id2]);
        } else {
            let mut iter_parts = line.split(": ");
            let id = iter_parts.next().expect("sane input").to_owned();
            let init = iter_parts
                .next()
                .expect("sane input")
                .parse::<u8>()
                .unwrap()
                == 1;
            inputs.insert(id, init);
        }
    }
    (inputs, gates)
}

fn do_part_one(mut state: HashMap<String, bool>, gates: &[[String; 4]]) -> u64 {
    simulate(&mut state, gates);
    into_zulu(state)
}

fn simulate(state: &mut HashMap<String, bool>, gates: &[[String; 4]]) {
    let mut active = gates.to_vec();
    let mut future = vec![];
    while !active.is_empty() {
        // println!("Remaining: {}", active.len());
        for [i0, op, i1, out] in active.drain(..) {
            if let Some(&ii00) = state.get(&i0) {
                if let Some(&ii11) = state.get(&i1) {
                    let outout = match op.as_str() {
                        "AND" => ii00 & ii11,
                        "OR" => ii00 | ii11,
                        "XOR" => ii00 ^ ii11,
                        _ => panic!("bad input"),
                    };
                    state.insert(out, outout);
                    continue;
                }
            }
            // not ready yet; try again later
            future.push([i0, op, i1, out]);
        }
        std::mem::swap(&mut active, &mut future);
    }
}

fn into_zulu(state: HashMap<String, bool>) -> u64 {
    state
        .iter()
        .filter(|(id, _val)| &id[0..1] == "z")
        .fold(0_u64, |acc, (id, &val)| {
            let index = id[1..].parse::<u32>().unwrap();
            acc | ((val as u64) << index)
        })
}

const SWAPPED_GATES: usize = 4;
#[allow(unused)]
fn do_part_two(mut gates: Vec<[String; 4]>) -> String {
    let mut swaps = vec![];
    for _ in 0..SWAPPED_GATES {
        let [i, j] = biggest_change(&mut gates);
        swaps.push(gates[i][3].clone());
        swaps.push(gates[j][3].clone());
        // keep that swap
        let (left, right) = gates.split_at_mut(j);
        std::mem::swap(&mut left[i][3], &mut right[0][3]);
    }
    swaps.sort_unstable();
    let mut retval = String::new();
    for id in swaps {
        retval.push_str(&id);
        retval.push(',');
    }
    let _trailing_comma = retval.pop();
    retval
}

fn biggest_change(gates: &mut [[String; 4]]) -> [usize; 2] {
    let mut fixed = u32::MAX;
    let mut best = [0, 0];
    for i in 0..gates.len() {
        for j in i + 1..gates.len() {
            {
                // swap outputs
                let (left, right) = gates.split_at_mut(j);
                std::mem::swap(&mut left[i][3], &mut right[0][3]);
            }
            let wrong = how_wrong(&gates);
            if wrong < fixed {
                fixed = wrong;
                best = [i, j];
            }
            {
                // swap back
                let (left, right) = gates.split_at_mut(j);
                std::mem::swap(&mut left[i][3], &mut right[0][3]);
            }
        }
    }
    // println!("Found {best:?}");
    best
}

fn how_wrong(gates: &[[String; 4]]) -> u32 {
    // making waves
    // Two 45-bit numbers go in, one 46-bit number comes out.
    let mut dominoes = HashMap::new();
    for pos in 0..45 {
        let x_id = format!("x{pos:02}");
        dominoes.insert(x_id, true);
        let y_id = format!("y{pos:02}");
        dominoes.insert(y_id, false);
    }
    dominoes.insert("y00".to_owned(), true);
    simulate(&mut dominoes, gates);
    let got = into_zulu(dominoes);
    let want: u64 = 1 << 45;
    let wrong = (want ^ got).count_ones();
    wrong
}

fn generate_graphviz(gates: &[[String; 4]], input_length: u32) -> String {
    // Rust is probably the wrong tool for this.
    let mut gv = String::new();
    gv.push_str("digraph aoc24 {\n");
    {
        for x in ['x', 'y', 'z'] {
            gv.push_str("subgraph external_");
            gv.push(x);
            gv.push_str(" { node [shape=circle,style=filled,color=yellow]; \n");
            let input_length = if x == 'z' {
                input_length + 1
            } else {
                input_length
            };
            for pos in 0..input_length {
                let id = format!("{x}{pos:02} -> ");
                gv.push_str(id.as_str());
            }
            gv.pop();
            gv.pop();
            gv.pop();
            gv.pop();
            gv.push_str(";\n}\n");
        }
    }
    let (mut ands, mut ors, mut xors, mut inputs) =
        (String::new(), String::new(), String::new(), String::new());
    for [i0, op, i1, out] in gates.iter() {
        inputs.push_str(format!("{i0} -> {out};\t").as_str());
        inputs.push_str(format!("{i1} -> {out};\n").as_str());
        match op.as_str() {
            "AND" => ands.push_str(format!(" {out};").as_str()),
            "OR" => ors.push_str(format!(" {out};").as_str()),
            "XOR" => xors.push_str(format!(" {out};").as_str()),
            _ => panic!("bad input"),
        }
    }
    gv.push_str("subgraph op_ands { node [style=filled,color=red]; \n");
    gv.push_str(ands.as_str());
    gv.push_str("\n}\n");
    gv.push_str("subgraph op_xors { node [style=filled,color=green]; \n");
    gv.push_str(xors.as_str());
    gv.push_str("\n}\n");
    gv.push_str("subgraph op_ors { node [style=filled,color=blue]; \n");
    gv.push_str(ors.as_str());
    gv.push_str("\n}\n");
    //
    gv.push_str(inputs.as_str());
    gv.push_str("}\n");
    gv
}
// Visually:
// bjm,z07
// skf,z18
// hsw,z13
// nvr,wkr
