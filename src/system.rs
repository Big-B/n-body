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

#[derive(Debug, Clone, Copy)]
pub struct Particle<'a> {
    name : &'a str,
    position: Point,
    vx : f64,
    vy : f64,
    vz : f64,
    fx : f64,
    fy : f64,
    fz : f64,
    mass : f64,
}

/// The gravitational constant as defined in wikipedia:
/// 6.67408(31)x10^(-11)m^(3)*kg^(-1)*s^(-2)
const G: f64 = 6.67408e-11;
impl<'a> Particle<'a> {
    pub fn new(name: &str, px : f64, py : f64, pz : f64,
               vx : f64, vy : f64, vz : f64,
               mass : f64) -> Particle {
        let point = Point::new(px, py, pz);
        Particle {name: name, position: point, vx: vx, vy: vy, vz: vz, fx: 0_f64,
        fy: 0_f64, fz: 0_f64, mass: mass}
    }

    pub fn add_particle_force(&mut self, other: &Particle) {
        let distance = self.position.distance(&other.position);
        let force = (G * self.mass * other.mass)/(distance.powi(3));
        self.fx += force * (other.position.x - self.position.x);
        self.fy += force * (other.position.y - self.position.y);
        self.fz += force * (other.position.z - self.position.z);
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
}

#[derive(Debug)]
pub struct System<'a> {
    particles: Vec<Particle<'a>>,
}

impl<'a> System<'a> {
    pub fn new() -> System<'a> {
        System {particles: Vec::new()}
    }

    pub fn add_particle(&mut self, particle: Particle<'a>) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, time: f64) {
        let len = self.particles.len();
        for i in 0..len {
            for j in 0..len {
                if i != j {
                    let other = self.particles[j];
                    self.particles[i].add_particle_force(&other);
                }
            }
        }

        for i in 0..len {
            self.particles[i].update(time);
        }
    }

    pub fn print(&self) {
        let len = self.particles.len();
        for i in 0..len {
            println!("{:?}", self.particles[i]);
        }
    }
}
