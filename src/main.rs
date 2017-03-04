mod particle;

const time: f64 = 0.000000001;

fn main() {
    let mut earth: particle::Particle = particle::Particle::new(1.4960e11, 0.0, 1.0,
                                                                0.0, 2.98e04, 0.0, 5.974e24);
    let mut sun: particle::Particle = particle::Particle::new(0.0, 0.0, 0.0,
                                                                0.0, 0.0, 0.0, 1.989e30);

    println!("Initial Particle");
    println!("earth: {:?}", earth);
    println!("sun: {:?}", sun);

    for _ in 0..1_000_000 {
        println!("Loop entrance point");
        earth.AddParticleForce(&sun);
        sun.AddParticleForce(&earth);
        println!("earth: {:?}", earth);
        println!("sun: {:?}", sun);
        earth.Update(time);
        sun.Update(time);
        println!("earth: {:?}", earth);
        println!("sun: {:?}", sun);
    }
}
