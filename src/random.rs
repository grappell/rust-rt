use rand::distr::{Distribution, Uniform}; // Ensure you import the correct Uniform type

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

    pub fn random_float_range(&mut self, min: f32, max: f32) -> f32 {
        let uniform = Uniform::new(min, max).unwrap(); // Use the Uniform distribution from the rand crate
        uniform.sample(&mut self.rng) // Sample from the distribution using self.rng
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

    pub fn random_unit_vector_on_sphere(&mut self) -> Vec3 {
        loop {
            let vec = self.random_vec3_range(-1.0, 1.0);
            let len_q = vec.length_squared();
            if 1e-160 < len_q && len_q < 1.0 {
                return vec / len_q.sqrt();
            }
        };
    }

    pub fn random_in_hemisphere(&mut self, normal: &Vec3) -> Vec3 {
        let in_unit_sphere = self.random_unit_vector_on_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }
}