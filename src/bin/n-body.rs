use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process;

use lazy_static::lazy_static;
use n_body::particle::Particle;
use n_body::point::Point;
use n_body::system::System;
use rayon::prelude::*;
use regex::Regex;
use std::io::{BufRead, Error, Write};
use std::sync::{Arc, Mutex};

const KM_TO_M: f64 = 1000.0;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Run {
        #[clap(short, long = "file", help = "Input file of particles")]
        file: PathBuf,
        #[clap(
            short,
            long = "duration",
            help = "Number of iterations to run the sim for",
            default_value_t = 3.154e7
        )]
        duration: f64,
        #[clap(
            short,
            long = "granularity",
            help = "Size of simulation steps in seconds",
            default_value_t = 1.0
        )]
        granularity: f64,
    },
    Download {
        #[clap(short, long = "output-file", help = "File to put downloaded data")]
        output: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Run {
            file,
            duration,
            granularity,
        } => {
            // Run command -- run the simulation
            let mut system = System::new();

            // Parse the file
            parse_file(&file, &mut system).unwrap_or_else(|err| {
                eprintln!("Failed to parse file: {}", err);
                process::exit(-1);
            });

            // Run the sim
            system.print();
            for _ in 0..duration as u64 {
                system.update(granularity);
            }
            system.print();
        }
        Action::Download { output } => {
            download_particles(&output).unwrap_or_else(|err| {
                eprintln!("Failed to download data: {}", err);
                process::exit(-1);
            });
        }
    }
}

fn parse_file<P: AsRef<Path>>(input: &P, system: &mut System) -> Result<(), Error> {
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

fn download_particles<P: AsRef<Path>>(output: &P) -> Result<(), Error> {
    // Open file
    let f = File::create(output)?;
    let writer = Arc::new(Mutex::new(BufWriter::new(f)));
    let count = Arc::new(Mutex::new(0));
    let client = reqwest::blocking::Client::new();
    let max = 1000;

    // Download command -- download the data
    (0..=max).into_par_iter().for_each(|i| {
        // Generate url for 1000 objects. Not all will be valid
        let url = format!("https://ssd.jpl.nasa.gov/horizons_batch.cgi?batch=1&COMMAND='{}'&MAKE_EPHEM='YES'&TABLE_TYPE='VECTOR'&START_TIME='2016-01-01'&STOP_TIME='2016-01-02'&STEP_SIZE='2%20d'&QUANTITIES='1,9,20,23,24'&CSV_FORMAT='YES'&CENTER='500@0'", i);

        // Grab the page
        let page: String = client.get(url).send().unwrap().text().unwrap();

        // Parse the mass data
        if let Ok(mass) = get_mass(&page) {
            // Parse the name
            if let Ok(name) = get_name(&page) {
                // Parse the position and velocity
                if let Ok(pos_vel) = get_pos_vel(&page, &name, mass) {
                    let mut writer = writer.lock().unwrap();
                    writer.write_all(serde_json::to_string(&pos_vel).unwrap().as_bytes()).unwrap();
                    writer.write_all(b"\n").unwrap();
                }
            }
        }

        // Increment count and provide update to user
        let mut count = count.lock().unwrap();
        *count += 1;
        print!("{}%\r", *count * 100 / max);
        io::stdout().flush().unwrap();
    });
    println!();
    Ok(())
}

fn get_name(lines: &str) -> Result<String, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Target\s*body\s*name:\s*([[:alnum:]]*)").unwrap();
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
            Regex::new(r"Mass.*\(?10\^(\d*) kg\s*\)?\s*[=~]\s*(\d+\.?\d*)").unwrap();
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
        static ref RE: Regex = Regex::new(r"(?:\-?\d*\.*\d*E[\+\-]\d*,\s+){6}").unwrap();
    }

    // Capture the values
    if let Some(cap) = RE.captures(lines) {
        // Grab the string and operate on it
        if let Some(val) = cap.get(0) {
            // Separate the strings into entries of a vector
            let mut nums: Vec<&str> = val.as_str().trim().split(',').collect();

            // There's always an extra empty one, get rid of it
            nums.pop();
            Ok(Particle::new(
                name.trim(),
                mass,
                Point::new(
                    nums[0].trim().parse::<f64>().unwrap() * KM_TO_M,
                    nums[1].trim().parse::<f64>().unwrap() * KM_TO_M,
                    nums[2].trim().parse::<f64>().unwrap() * KM_TO_M,
                ),
                nums[3].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[4].trim().parse::<f64>().unwrap() * KM_TO_M,
                nums[5].trim().parse::<f64>().unwrap() * KM_TO_M,
            ))
        } else {
            Err("Position/Velocity Regex Error".to_string())
        }
    } else {
        Err("Position/Velocity Regex Error".to_string())
    }
}
