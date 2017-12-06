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

    let new_ninfo = {
        let contacts = ninfo.contacts
                            .iter()
                            .filter(|c|
                                    c.pair.0.index >= 114 &&
                                    c.pair.0.index <= 174 &&
                                    c.pair.1.index >= 114 &&
                                    c.pair.1.index <= 174)
                            .map(|c| c.clone())
                            .collect();

        NativeInfo {
            bonds: Vec::new(),
            angles: Vec::new(),
            dihedral_angles: Vec::new(),
            contacts: contacts,
            aicg_angles: Vec::new(),
            aicg_dihedral_angles: Vec::new(),
        }
    };

    println!("{}", new_ninfo);
}
