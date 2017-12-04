extern crate cafetools;
extern crate dcdio;

use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use cafetools::native_info::NativeInfo;
use dcdio::DcdReader;

fn distance(x: &(f32, f32, f32), y: &(f32, f32, f32)) -> f32 {
    ((x.0 - y.0).powi(2) + (x.1 - y.1).powi(2) + (x.2 - y.2).powi(2)).sqrt()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        let program = &args[0];
        eprintln!("Usage: {} NINFO STRUCTURE", program);
        process::exit(1);
    }

    let ninfo = {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);
        NativeInfo::load(reader).unwrap()
    };

    let dcdreader = {
        let file = File::open(&args[2]).unwrap();
        DcdReader::new(file).unwrap()
    };

    let contacts = {
        let mut contacts = ninfo.contacts;
        contacts.sort_by(|x, y| x.index.cmp(&y.index));
        contacts
    };

    println!("time\tqscore");

    for frame in dcdreader.frames() {
        let frame = frame.unwrap();
        let mut num_contacts = 0;

        for contact in &contacts {
            let &(ref index0, ref index1) = &contact.pair;
            let dist = distance(&frame.positions[index0.index-1],
                                &frame.positions[index1.index-1]) as f64;
            if dist < contact.length {
                num_contacts += 1;
            }
        }

        println!("{}\t{}", frame.time, num_contacts as f64 / contacts.len() as f64);
    }
}
