#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point{x: x, y: y, z: z}
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)
         + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug)]
pub struct Particle {
    position: Point,
    vx : f64,
    vy : f64,
    vz : f64,
    fx : f64,
    fy : f64,
    fz : f64,
    mass : f64,
}

const G: f64 = 6.67408e11;
impl Particle {
    pub fn new(px : f64, py : f64, pz : f64,
               vx : f64, vy : f64, vz : f64,
               mass : f64) -> Particle {
        let point = Point::new(px, py, pz);
        Particle {position: point, vx: vx, vy: vy, vz: vz, fx: 0_f64,
        fy: 0_f64, fz: 0_f64, mass: mass}
    }

    pub fn AddParticleForce(&mut self, other: &Particle) {
        let distance = self.position.distance(&other.position);
        let force = (G * self.mass * other.mass)/(distance.powi(3));
        self.fx += force * (other.position.x - self.position.x) / distance;
        self.fy += force * (other.position.y - self.position.y) / distance;
        self.fz += force * (other.position.z - self.position.z) / distance;
    }

    pub fn Update(&mut self, time: f64) {
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
