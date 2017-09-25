use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;

fn print_usage(program: &str) {
    println!("Usage: {} INPUT OUTPUT", program);
}

fn skip_lines<R: BufRead>(reader: &mut R, num_lines: usize) -> std::io::Result<()> {
    let mut buf = String::new();
    for _ in 0..num_lines {
        reader.read_line(&mut buf)?;
    }
    Ok(())
}

fn replace_tab_to_comma(line: &str) -> String {
    line.split_whitespace()
        .collect::<Vec<&str>>()
        .join(",")
}

fn convert_all<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    skip_lines(reader, 5)?;

    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    writeln!(writer, "{}", replace_tab_to_comma(&buf[6..]))?;

    skip_lines(reader, 3)?;

    for line in reader.lines() {
        let line = line?;
        if line.len() < 5 || line[..5].trim().len() == 0 {
            continue;
        }
        writeln!(writer, "{}", replace_tab_to_comma(&line[6..]))?;
    }

    Ok(())
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
