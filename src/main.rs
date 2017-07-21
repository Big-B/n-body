#![feature(iterator_for_each)]
extern crate getopts;
extern crate rayon;
use getopts::Options;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

mod system;
use system::System;
use system::Particle;
use system::Point;
use std::io::BufRead;
use std::io::Error;

const TIME: f64 = 1.0000;
const DURATION: u64 = 3.154e7 as u64;
const AU_TO_M: f64 = 149597870700.0;
const DAY_TO_SEC: f64 = 86400.0;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_file(input: &str, system: &mut System) -> Result<(), Error> {

    // Open file
    let f = File::open(input)?;

    // Use a bufreader to read lines
    let reader = BufReader::new(f);
    for line in reader.lines() {

        // Read out the line as a str and split it on whitespace
        let string = line.unwrap();
        let vec: Vec<&str> = string.split_whitespace().collect();

        // Pull out all the fields
        let name: &str = vec[0];
        let mass: f64 = vec[1].parse().unwrap();
        let x: f64 = vec[2].parse::<f64>().unwrap() * AU_TO_M;
        let y: f64 = vec[3].parse::<f64>().unwrap() * AU_TO_M;
        let z: f64 = vec[4].parse::<f64>().unwrap() * AU_TO_M;
        let vx: f64 = vec[5].parse::<f64>().unwrap() * AU_TO_M / DAY_TO_SEC;
        let vy: f64 = vec[6].parse::<f64>().unwrap() * AU_TO_M / DAY_TO_SEC;
        let vz: f64 = vec[7].parse::<f64>().unwrap() * AU_TO_M / DAY_TO_SEC;

        let point: Point = Point::new(x, y, z);

        // Create a new particle
        let part: Particle = Particle::new(name, mass, point,
                                           vx, vy, vz);

        // Place the particle into the system
        system.add_particle(part);
    }
    Ok(())
}

fn main() {
    // Create a system
    let mut system = System::new();

    // Collect commandline info
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Setup commandline options
    let mut opts = Options::new();
    opts.optopt("f", "file", "input file", "NAME");
    opts.optflag("h", "help", "print this help menu");

    // Parse the arguments
    let matches = opts.parse(&args[1..]).unwrap_or_else(|err| {
        eprintln!("Argument Parsing Error: {}", err);
        print_usage(&program, &opts);

        process::exit(-1);
    });

    // Check for help flag
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(0);
    }

    // Get input file
    let input = matches.opt_str("f").unwrap_or_else(|| {
        eprintln!("A filename is required");
        process::exit(-1);
    });

    // Parse the file
    parse_file(&input, &mut system).unwrap_or_else(|err| {
        eprintln!("Failed to parse file: {}", err);
        process::exit(-1);
    });

    // Run the simulation
    system.print();
    for _ in 0..DURATION {
        system.update(TIME);
    }
    system.print();
}
