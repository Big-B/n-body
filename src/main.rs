mod system;

const TIME: f64 = 1.0000;
const DURATION: u64 = 3.154e7 as u64;

fn main() {
    let earth: system::Particle = system::Particle::new(1.4960e11, 0.0, 1.0,
                                                            0.0, 2.98e04, 0.0, 5.972e24);
    let sun: system::Particle = system::Particle::new(0.0, 0.0, 0.0,
                                                          0.0, 0.0, 0.0, 1.989e30);
    let mut system: system::System = system::System::new();
    system.add_particle(sun);
    system.add_particle(earth);

    system.print();
    for _ in 0..DURATION {
        system.update(TIME);
    }
    system.print();
}
