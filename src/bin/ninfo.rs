extern crate cafetools;
extern crate dcdio;

use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use cafetools::native_info::NativeInfo;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let program = &args[0];
        eprintln!("Usage: {} NINFO", program);
        process::exit(1);
    }

    let ninfo = {
        let file = File::open(&args[1]).unwrap();
        let reader = BufReader::new(file);
        NativeInfo::load(reader).unwrap()
    };

    println!("index,pid0,pid1,length,coefficient,type");
    for contact in ninfo.contacts.into_iter() {
        println!("{},{},{},{},{},{}",
                 contact.index,
                 contact.pair.0.index,
                 contact.pair.1.index,
                 contact.length,
                 contact.coefficient,
                 contact.ty);
    }
}
