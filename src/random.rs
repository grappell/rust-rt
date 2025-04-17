use rand::Rng;

use crate::util::Vec3;

pub struct RandomGenerator {
    rng: rand::rngs::ThreadRng,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            rng: rand::rng(),
        }
    }

    pub fn random_float(&mut self) -> f32 {
        self.rng.random()
    }

    pub fn random_float_range(&mut self, min: f32, max: f32) -> f32 {
        self.rng.random_range(min..max)
    }

    pub fn random_vec3(&mut self) -> Vec3 {
        Vec3::new(self.random_float(), self.random_float(), self.random_float())
    }

    pub fn random_vec3_range(&mut self, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            self.random_float_range(min, max),
            self.random_float_range(min, max),
            self.random_float_range(min, max),
        )
    }

    pub fn random_vec3_square(&mut self) -> Vec3 {
        Vec3::new(
            self.random_float_range(-0.5, 0.5),
            self.random_float_range(-0.5, 0.5),
            0.0,
        )
    }
}