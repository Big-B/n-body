mod particle;

const TIME: f64 = 0.000000001;

fn main() {
    let mut earth: particle::Particle = particle::Particle::new(1.4960e11, 0.0, 1.0,
                                                                0.0, 2.98e04, 0.0, 5.974e24);
    let mut sun: particle::Particle = particle::Particle::new(0.0, 0.0, 0.0,
                                                              0.0, 0.0, 0.0, 1.989e30);
    println!("earth: {:?}", earth);
    println!("sun: {:?}", sun);

    for _ in 0..1_000_000 {
        earth.add_particle_force(&sun);
        sun.add_particle_force(&earth);
        earth.update(TIME);
        sun.update(TIME);
    }
    println!("earth: {:?}", earth);
    println!("sun: {:?}", sun);
}
