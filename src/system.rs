use particle::Particle;

#[derive(Debug, Default)]
pub struct System {
    particles0: Vec<Particle>,
    particles1: Vec<Particle>,
}

impl System {
    pub fn new() -> System {
        System {
            particles0: Vec::new(),
            particles1: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        // We need a copy so that we don't have to
        // create and destroy a copy every update
        // iteration
        self.particles0.push(particle.clone());
        self.particles1.push(particle);
    }

    pub fn update(&mut self, time: f64) {
        {
            // Take each particle, calculate the forces from
            // all the other articles, and add them
            let (first, second) = (&mut self.particles0, &self.particles1);
            first.iter_mut().for_each(|part0| {
                second.iter().for_each(move |part1| {
                    part0.add_particle_force(part1);
                })
            });
        }

        {
            // Update the particles and make the vectors identical again
            let (first, second) = (&mut self.particles0, &mut self.particles1);
            second
                .iter_mut()
                .zip(&mut first[..])
                .for_each(|(sec, fir)| {
                    fir.update(time);
                    sec.set_equal(fir);
                });
        }
    }

    pub fn print(&self) {
        for part in &self.particles0 {
            println!("{}\n", part);
        }
    }
}
