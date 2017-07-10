use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point{x: x, y: y, z: z}
    }

    /// Calculate the distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)
         + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    name : String,
    mass : f64,
    position: Point,
    vx : f64,
    vy : f64,
    vz : f64,
    fx : f64,
    fy : f64,
    fz : f64,
}

/// The gravitational constant as defined in wikipedia:
/// 6.67408(31)x10^(-11)m^(3)*kg^(-1)*s^(-2)
const G: f64 = 6.67408e-11;
impl Particle {
    pub fn new(name: &str, mass : f64, point: Point,
               vx : f64, vy : f64, vz : f64) -> Particle {
        Particle {name: name.to_string(), mass: mass, position: point,
        vx: vx, vy: vy, vz: vz, fx: 0_f64, fy: 0_f64, fz: 0_f64}
    }

    pub fn add_particle_force(&mut self, other: &Particle) {
        let distance = self.position.distance(&other.position);
        if distance != 0_f64 {
            let force = (G * self.mass * other.mass)/(distance.powi(3));
            self.fx += force * (other.position.x - self.position.x);
            self.fy += force * (other.position.y - self.position.y);
            self.fz += force * (other.position.z - self.position.z);
        } else {
            self.fx += 0_f64;
            self.fy += 0_f64;
            self.fz += 0_f64;
        }
    }

    pub fn update(&mut self, time: f64) {
        self.vx += time*self.fx/self.mass;
        self.vy += time*self.fy/self.mass;
        self.vz += time*self.fz/self.mass;

        self.position.x += time*self.vx;
        self.position.y += time*self.vy;
        self.position.z += time*self.vz;

        self.fx = 0_f64;
        self.fy = 0_f64;
        self.fz = 0_f64;
    }

    pub fn set_equal(&mut self, other: &Particle) {
        self.position.x = other.position.x;
        self.position.y = other.position.y;
        self.position.z = other.position.z;

        self.vx = other.vx;
        self.vy = other.vy;
        self.vz = other.vz;
    }
}

#[derive(Debug)]
pub struct System {
    particles0: Vec<Particle>,
    particles1: Vec<Particle>,
}

impl System {
    pub fn new() -> System {
        System {particles0: Vec::new(), particles1: Vec::new()}
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles0.push(particle.clone());
        self.particles1.push(particle);
    }

    pub fn update(&mut self, time: f64) {
        {
            let (first, second) = (&mut self.particles0, &self.particles1);
            first.par_iter_mut().for_each(|part0| {
                second.iter().for_each(move |part1| {
                    part0.add_particle_force(part1);
                })
            });
        }

        {
            let (first, second) = (&mut self.particles0, &mut self.particles1);
            second.par_iter_mut().zip(&mut first[..])
                .for_each(|(sec, fir)| {
                    fir.update(time);
                    sec.set_equal(&fir);
                });
        }
    }

    pub fn print(&self) {
        for part in &self.particles0 {
            println!("{:?}", part);
        }
    }
}
