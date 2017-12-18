extern crate dcdio;

use std::env;
use std::process;
use std::fs::File;
use dcdio::DcdReader;

type Vector3d = (f32, f32, f32);

fn add(x: Vector3d, y: &Vector3d) -> Vector3d {
    (x.0 + y.0, x.1 + y.1, x.2 + y.2)
}

fn sub(x: Vector3d, y: Vector3d) -> Vector3d {
    (x.0 - y.0, x.1 - y.1, x.2 - y.2)
}

fn length_sq(x: Vector3d) -> f32 {
    x.0.powi(2) + x.1.powi(2) + x.2.powi(2)
}

fn div(x: Vector3d, y: f32) -> Vector3d {
    (x.0 / y, x.1 / y, x.2 / y)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let program = &args[0];
        eprintln!("Usage: {} STRUCTURE", program);
        process::exit(1);
    }

    let dcdreader = DcdReader::new(File::open(&args[1]).unwrap()).unwrap();

    println!("time\trg");
    for frame in dcdreader.frames() {
        let frame = frame.unwrap();
        let num_particles = frame.positions.len() as f32;
        let mean = div(frame.positions.iter().fold((0.0, 0.0, 0.0), add),
                       num_particles);

        let mut rg = 0.0;
        for position in frame.positions {
            rg += length_sq(sub(position, mean));
        }
        println!("{}\t{}", frame.time, (rg / num_particles).sqrt());
    }
}
