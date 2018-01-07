extern crate dcdio;

use std::env;
use std::process;
use std::fs::File;
use dcdio::DcdReader;

const ADDITIONAL_LENGTH: usize = 43;

fn distance(x: &(f32, f32, f32), y: &(f32, f32, f32)) -> f32 {
    ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2) + (x.2 - y.2).powi(2)).sqrt()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let program = &args[0];
        eprintln!("Usage: {} STRUCTURE", program);
        process::exit(1);
    }

    let dcdreader = DcdReader::new(File::open(&args[1]).unwrap()).unwrap();

    // for frame in dcdreader.frames() {
    if let Some(frame) = dcdreader.frames().last() {
        let frame = frame.unwrap();
        let num = frame.positions.len() - ADDITIONAL_LENGTH;

        for i in 0..num {
            let pos_i = &frame.positions[i];
            for j in 0..num {
                let pos_j = &frame.positions[j];
                println!("{},{},{}", i, j, distance(pos_i, pos_j));
            }
            println!();
        }
    }
}

