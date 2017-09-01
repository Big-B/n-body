#![feature(iterator_for_each)]
#![feature(inclusive_range_syntax)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate regex;

use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::process;

mod system;
use system::{System, Particle, Point};
use std::io::{BufRead, Error, Read, Write};
use regex::Regex;

use clap::{Arg, App, SubCommand};

const TIME: f64 = 1.0000;
const DURATION: u64 = 3.154e7 as u64;
const KM_TO_M: f64 = 1000.0;

fn main() {
    let matches = App::new("n-body")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .subcommand(SubCommand::with_name("run")
                    .about("run the simulation")
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .value_name("FILE")
                         .help("Input files of particles")
                         .required(true)
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("download")
                    .about("download solar system data")
                    .arg(Arg::with_name("output")
                         .short("o")
                         .long("output-file")
                         .value_name("OUT")
                         .help("File to put downloaded data")
                         .required(true)
                         .takes_value(true)))
        .get_matches();


    // Which mode are we running in ?
    if let Some(matches) = matches.subcommand_matches("run")
    {
        // Run command -- run the simulation
        // Unwrap because it's a required argument
        let file = matches.value_of("file").unwrap();
        let mut system = System::new();

        // Parse the file
        parse_file(&file, &mut system).unwrap_or_else(|err| {
            eprintln!("Failed to parse file: {}", err);
            process::exit(-1);
        });

        // Run the sim
        system.print();
        for _ in 0..DURATION {
            system.update(TIME);
        }
        system.print();

    } else if let Some(matches) = matches.subcommand_matches("download") {
        download_particles(matches.value_of("output").unwrap())
            .unwrap_or_else(|err| {
                eprintln!("Failed to download data: {}", err);
                process::exit(-1);
            });
    } else {
        // Not a valid mode, print error and usage
        eprintln!("{}", matches.usage());
        process::exit(-1);
    }
}

fn parse_file(input: &str, system: &mut System) -> Result<(), Error> {

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

fn download_particles(output: &str) -> Result<(), Error> {
    // Open file
    let f = File::create(output)?;
    let mut writer = BufWriter::new(f);

    // Download command -- download the data
    for i in 0...1000 {
        // Generate url for 1000 objects. Not all will be valid
        let url = format!("https://ssd.jpl.nasa.gov/horizons_batch.cgi?batch=1&COMMAND='{}'&MAKE_EPHEM='YES'&TABLE_TYPE='VECTOR'&START_TIME='2016-01-01'&STOP_TIME='2016-01-02'&STEP_SIZE='2%20d'&QUANTITIES='1,9,20,23,24'&CSV_FORMAT='YES'&CENTER='500@0'", i);

        // Grab the page
        let mut page = String::new();
        reqwest::get(&url).unwrap().read_to_string(&mut page).unwrap();

        // Parse the mass data
        if let Ok(mass) = get_mass(&page) {
            // Parse the name
            if let Ok(name) = get_name(&page) {
                // Parse the position and velocity
                if let Ok(pos_vel) = get_pos_vel(&page, &name, mass) {
                    writer.write(serde_json::to_string(&pos_vel).unwrap().as_bytes())?;
                    writer.write(b"\n")?;
                }
            }
        }
        print!("{}%\r", i/10);
        io::stdout().flush()?;
    }
    println!("");
    Ok(())
}

fn get_name(lines: &str) -> Result<String, String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Target\s*body\s*name:\s*([[:alnum:]]*)")
            .unwrap();
    }

    if let Some(cap) = RE.captures(lines) {
        if let Some(name) = cap.get(1) {
            Ok(name.as_str().to_string())
        } else {
            Err("Name regex error".to_string())
        }
    } else {
        Err("Name regex error".to_string())
    }
}

fn get_mass(lines: &str) -> Result<f64, String> {
    // Regex to capture just the mass field. It has two fields, a base
    // numer and a magnitude/exponent. It looks something like this:
    // Mass (10^23 kg ) = 6.4185
    // We only need to know the exponent (23) and the base value that
    // we'll multiply it to.
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Mass.*\(?10\^(\d*) kg\s*\)?\s*[=~]\s*(\d+\.?\d*)")
            .unwrap();
    }

    // Search all the input
    if let Some(cap) = RE.captures(lines) {
        // Make sure we got matches
        if let (Some(base), Some(exponent)) = (cap.get(2), cap.get(1)) {
            let mass = format!("{}E{}", base.as_str(), exponent.as_str());

            let mass = mass.parse::<f64>().unwrap();
            // Return value
            Ok(mass)
        } else {
            Err("Couldn't get values".to_string())
        }
    } else {
        Err("Couldn't get regex".to_string())
    }
}

fn get_pos_vel(lines: &str, name: &str, mass: f64) -> Result<Particle, String> {
    // Regex to capture the x,y,z,vx,vy,vz fields. It has six fields
    // that are numbers of the form -1.436E+09. The numbers can be
    // positive or negative as well as the exponents.
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?:\-?\d*\.*\d*E[\+\-]\d*,\s+){6}")
            .unwrap();
    }

    // Capture the values
    if let Some(cap) = RE.captures(lines) {
        // Grab the string and operate on it
        if let Some(val) = cap.get(0) {
            // Separate the strings into entries of a vector
            let mut nums: Vec<&str> = val.as_str().trim().split(',').collect();

            // There's always an extra empty one, get rid of it
            nums.pop();
            Ok(Particle::new(name.trim(), mass,
                Point::new(nums[0].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[1].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[2].trim().parse::<f64>().unwrap() * KM_TO_M),
                nums[3].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[4].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[5].trim().parse::<f64>().unwrap() * KM_TO_M))
        } else {
            Err("Position/Velocity Regex Error".to_string())
        }
    } else {
        Err("Position/Velocity Regex Error".to_string())
    }
}
