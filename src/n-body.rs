#![feature(iterator_for_each)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate getopts;
use getopts::Options;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
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

    let path = Path::new("particles2.txt");

    // Open file
    let f = File::open(input)?;
    let reader = BufReader::new(f);

    // Use a bufreader to read lines
    for line in reader.lines() {
        let part: Particle = serde_json::from_str(&line.unwrap())?;

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
