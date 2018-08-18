use point::Point;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Particle {
    name: String,
    mass: f64,
    position: Point,
    vx: f64,
    vy: f64,
    vz: f64,
    fx: f64,
    fy: f64,
    fz: f64,
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "Mass: {}", self.mass)?;
        writeln!(f, "Position X: {}", self.position.x)?;
        writeln!(f, "Position Y: {}", self.position.y)?;
        writeln!(f, "Position Z: {}", self.position.z)?;
        writeln!(f, "Velocity X: {}", self.vx)?;
        writeln!(f, "Velocity Y: {}", self.vy)?;
        write!(f, "Velocity Z: {}", self.vz)?;
        Ok(())
    }
}

/// The gravitational constant as defined in wikipedia:
/// 6.67408(31)x10^(-11)m^(3)*kg^(-1)*s^(-2)
const G: f64 = 6.67408e-11;
impl Particle {
    pub fn new(name: &str, mass: f64, point: Point, vx: f64, vy: f64, vz: f64) -> Particle {
        Particle {
            name: name.to_string(),
            mass,
            position: point,
            vx,
            vy,
            vz,
            fx: 0_f64,
            fy: 0_f64,
            fz: 0_f64,
        }
    }

    pub fn add_particle_force(&mut self, other: &Particle) {
        let distance = self.position.distance(&other.position);
        if distance != 0_f64 {
            let force = (G * self.mass * other.mass) / (distance.powi(3));
            self.fx += force * (other.position.x - self.position.x);
            self.fy += force * (other.position.y - self.position.y);
            self.fz += force * (other.position.z - self.position.z);
        }
    }

    pub fn update(&mut self, time: f64) {
        self.vx += time * self.fx / self.mass;
        self.vy += time * self.fy / self.mass;
        self.vz += time * self.fz / self.mass;

        self.position.x += time * self.vx;
        self.position.y += time * self.vy;
        self.position.z += time * self.vz;

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

    pub fn get_position(&self) -> Point {
        self.position
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn set_equal_position() {
        let (x, y, z) = (0_f64, 1_f64, 2_f64);
        let p1 = Point::new(x, y, z);
        let p2 = Point::new(x + 1_f64, y + 2_f64, z + 3_f64);
        let part1 = Particle::new("part1", 0_f64, p1, x, y, z);
        let mut part2 = Particle::new("part2", 0_f64, p2, x, y, z);
        part2.set_equal(&part1);
        assert_eq!(part1.position.x, part2.position.x);
        assert_eq!(part1.position.y, part2.position.y);
        assert_eq!(part1.position.z, part2.position.z);
    }

    #[bench]
    fn bench_add_particle_force(b: &mut Bencher) {
        let point = Point::new(1_f64, -2_f64, 3_f64);
        let test_point = Point::new(0_f64, 0_f64, 0_f64);
        let part = Particle::new("other", 10_f64, point, 1_f64, -2_f64, 3_f64);
        let mut test = Particle::new("test", 10_f64, test_point, -1_f64, 2_f64, -3_f64);
        b.iter(|| test.add_particle_force(&part));
    }

    #[bench]
    fn bench_update(b: &mut Bencher) {
        let test_point = Point::new(0_f64, 0_f64, 0_f64);
        let mut test = Particle::new("test", 10_f64, test_point, -1_f64, 2_f64, -3_f64);
        b.iter(|| test.update(1.1));
    }
}
