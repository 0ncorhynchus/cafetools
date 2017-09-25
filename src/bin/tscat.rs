extern crate cafetools;

use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use cafetools::skip_lines;

fn print_usage(program: &str) {
    let description = "Concatenate time-series FILE(s) to standard output.";

    println!("Usage: {} FILE...", program);
    println!("{}", description);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        process::exit(1);
    }

    {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }

    for filename in args[2..].iter() {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        skip_lines(&mut reader, 9).unwrap();

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}
