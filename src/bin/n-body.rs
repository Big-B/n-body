#![feature(euclidean_division)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate n_body;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

// Graphics
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::process;

use n_body::particle::Particle;
use n_body::point::Point;
use n_body::system::System;
use regex::Regex;
use std::io::{BufRead, Error, Read, Write};

use clap::{App, Arg, SubCommand};

const KM_TO_M: f64 = 1000.0;

pub struct PApp {
    gl: GlGraphics,
}

impl PApp {
    fn render(&mut self, args: &RenderArgs, system: &System, x_max: f64, y_max: f64, mass_max: f64) {
        use graphics::*;

        let (x, y) = ((args.width / 2) as f64,
        (args.height / 2) as f64);

        let n_x = x_max / x;
        let n_y = y_max / y;
        let n_mass = 100.0 * mass_max / x.max(y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(color::BLACK, gl);
            system.get_particles().iter().for_each(|part| {
                let ellipse = ellipse::circle(0.0, 0.0, 5.0_f64.max(part.get_mass() / n_mass));
                let x_pos = part.get_position().x / n_x;
                let y_pos = part.get_position().y / n_y;
                let transform = c.transform.trans(x, y)
                    .trans(x_pos, y_pos);

                // Draw an
                graphics::ellipse(color::WHITE, ellipse, transform, gl);
            });
        });
    }

    fn update(&mut self, args: &UpdateArgs, system: &mut System, granularity: f64) {
        for _ in 0..2000 {
            system.update(granularity);
        }
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("run")
                .about("run the simulation")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .help("Input files of particles")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("duration")
                        .short("d")
                        .long("duration")
                        .value_name("ITERATIONS")
                        .help("Number of iterations to run the sim for")
                        .required(false)
                        .takes_value(true)
                        .default_value("3.154e7"),
                )
                .arg(
                    Arg::with_name("granularity")
                        .short("g")
                        .long("granularity")
                        .value_name("SECONDS")
                        .help("Size of simulation steps in seconds")
                        .required(false)
                        .takes_value(true)
                        .default_value("1"),
                ).arg(
                    Arg::with_name("visual")
                        .short("V")
                        .long("visual")
                        .help("Display simulation"),
                ),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("download solar system data")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output-file")
                        .value_name("OUT")
                        .help("File to put downloaded data")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    // Which mode are we running in ?
    if let Some(matches) = matches.subcommand_matches("run") {
        // Run command -- run the simulation
        // Unwrap because it's a required argument
        let file = matches.value_of("file").unwrap();
        let mut system = System::new();

        // Parse the file
        parse_file(file, &mut system).unwrap_or_else(|err| {
            eprintln!("Failed to parse file: {}", err);
            process::exit(-1);
        });

        // Get the sim duration and granularity arguments
        let duration = matches
            .value_of("duration")
            .unwrap()
            .trim()
            .parse::<f64>()
            .unwrap();
        let granularity = matches
            .value_of("granularity")
            .unwrap()
            .trim()
            .parse::<f64>()
            .unwrap();

        if matches.is_present("visual") {
            draw_window(&mut system, granularity);
        } else {

            // Run the sim
            system.print();
            for _ in 0..duration as u64 {
                system.update(granularity);
            }
            system.print();
        }
    } else if let Some(matches) = matches.subcommand_matches("download") {
        download_particles(matches.value_of("output").unwrap()).unwrap_or_else(|err| {
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

fn draw_window(system: &mut System, granularity: f64) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let x_max = system.get_particles().iter().map(|part| {
        part.get_position().x
    }).fold(0./0., f64::max);

    let y_max = system.get_particles().iter().map(|part| {
        part.get_position().y
    }).fold(0./0., f64::max);

    let mass_max = system.get_particles().iter().map(|part| {
        part.get_mass()
    }).fold(0./0., f64::max);

    let mut app = PApp {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, system, x_max, y_max, mass_max);
        }

        if let Some(u) = e.update_args() {
            app.update(&u, system, granularity);
        }
    }
}

fn download_particles(output: &str) -> Result<(), Error> {
    // Open file
    let f = File::create(output)?;
    let mut writer = BufWriter::new(f);

    // Download command -- download the data
    for i in 0..=1000 {
        // Generate url for 1000 objects. Not all will be valid
        let url = format!("https://ssd.jpl.nasa.gov/horizons_batch.cgi?batch=1&COMMAND='{}'&MAKE_EPHEM='YES'&TABLE_TYPE='VECTOR'&START_TIME='2016-01-01'&STOP_TIME='2016-01-02'&STEP_SIZE='2%20d'&QUANTITIES='1,9,20,23,24'&CSV_FORMAT='YES'&CENTER='500@0'", i);

        // Grab the page
        let mut page = String::new();
        reqwest::get(&url)
            .unwrap()
            .read_to_string(&mut page)
            .unwrap();

        // Parse the mass data
        if let Ok(mass) = get_mass(&page) {
            // Parse the name
            if let Ok(name) = get_name(&page) {
                // Parse the position and velocity
                if let Ok(pos_vel) = get_pos_vel(&page, &name, mass) {
                    writer.write_all(serde_json::to_string(&pos_vel).unwrap().as_bytes())?;
                    writer.write_all(b"\n")?;
                }
            }
        }
        print!("{}%\r", i / 10);
        io::stdout().flush()?;
    }
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
