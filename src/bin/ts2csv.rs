extern crate cafetools;

use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;
use cafetools::skip_lines;
use cafetools::error::*;
use cafetools::time_series::*;

fn write_header<W: Write+?Sized>(writer: &mut W) -> std::io::Result<()> {
    writeln!(writer, "step,tempk,radg,etot,velet,qscore,rmsd")
}

fn write_snapshot<W: Write+?Sized>(writer: &mut W, snapshot: &SnapShot) -> std::io::Result<()> {
    writeln!(writer, "{},{:.2},{:.2},{:.2},{:.2},{:.3},{:.2}",
             snapshot.step,
             snapshot.tempk,
             snapshot.radg,
             snapshot.etot,
             snapshot.velet,
             snapshot.qscore,
             snapshot.rmsd)
}

fn convert_all<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> Result<()> {
    write_header(writer)?;

    skip_lines(reader, 9)?; // Skip headers

    for line in reader.lines() {
        let snapshot = line?.parse::<SnapShot>()?;
        if !snapshot.unit.is_empty() {
            continue;
        }
        write_snapshot(writer, &snapshot)?;
    }

    Ok(())
}

fn print_usage(program: &str) {
    println!("Usage: {} INPUT OUTPUT", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 3 {
        print_usage(&program);
        process::exit(1);
    }

    let infile = File::open(&args[1]).unwrap();
    let outfile = File::create(&args[2]).unwrap();

    let mut reader = BufReader::new(infile);
    let mut writer = LineWriter::new(outfile);

    convert_all(&mut reader, &mut writer).unwrap();
}
