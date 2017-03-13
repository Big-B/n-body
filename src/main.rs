mod system;

const TIME: f64 = 1.0000;
const DURATION: u64 = 3.154e7 as u64;

fn main() {
    let sun: system::Particle = system::Particle::new("Sun", 0.0, 0.0, 0.0,
                                                          0.0, 0.0, 0.0, 1.989e30);
    let mercury: system::Particle = system::Particle::new("Mercury", -1.1708e10, -5.7384e10, 0.0,
                                                          4.6276e4, -9.9541e3, 0.0, 3.3020e23);
    let venus: system::Particle = system::Particle::new("Venus", 6.9283e10, 8.2658e10, 0.0,
                                                        -2.6894e4, 2.2585e4, 0.0, 4.8690e24);
    let earth: system::Particle = system::Particle::new("Earth", 1.4960e11, 0.0, 0.0,
                                                            0.0, 2.98e04, 0.0, 5.972e24);
    let mars: system::Particle = system::Particle::new("Mars", -1.1055e11, -1.9868e11, 0.0,
                                                       2.1060e4, -1.1827e4, 0.0, 6.4190e23);
    let mut system: system::System = system::System::new();
    system.add_particle(sun);
    system.add_particle(mercury);
    system.add_particle(venus);
    system.add_particle(earth);
    system.add_particle(mars);

    system.print();
    for _ in 0..DURATION {
        system.update(TIME);
    }
    system.print();
}
