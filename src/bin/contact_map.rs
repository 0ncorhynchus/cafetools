extern crate dcdio;

use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use dcdio::DcdReader;

const ADDITIONAL_LENGTH: usize = 43;

type ReferenceMap = HashMap<(usize, usize), f32>;

fn read_reference(filename: &str) -> std::io::Result<ReferenceMap> {
    let reader = BufReader::new(File::open(filename)?);
    let mut map = ReferenceMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let split: Vec<_> = line.split_whitespace().collect();
        let key = (split[0].parse().unwrap(),
                   split[1].parse().unwrap());
        map.insert(key, split[2].parse().unwrap());
    }

    Ok(map)
}

fn distance(x: &(f32, f32, f32), y: &(f32, f32, f32)) -> f32 {
    ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2) + (x.2 - y.2).powi(2)).sqrt()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        let program = &args[0];
        eprintln!("Usage: {} REFERENCE STRUCTURE INDEX", program);
        process::exit(1);
    }

    let refmap = read_reference(&args[1]).unwrap();
    let dcdreader = DcdReader::new(File::open(&args[2]).unwrap()).unwrap();
    let index = args[3].parse().unwrap();

    if let Some(frame) = dcdreader.frames().nth(index) {
        let frame = frame.unwrap();
        let num = frame.positions.len() - ADDITIONAL_LENGTH;

        for i in 0..num {
            let pos_i = &frame.positions[i];

            let strings: Vec<_>  = (0..num).map(|j| {
                match refmap.get(&(i+1, j+1)) {
                    Some(reference) => {
                        let pos_j = &frame.positions[j];
                        (distance(pos_i, pos_j) - reference).to_string()
                    },
                    None => {
                        "".to_string()
                    }
                }
            }).collect();

            println!("{}", strings.join(","));
        }
    }
}
