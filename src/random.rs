use rand::distr::{Distribution, SampleString, Uniform};
use rand_distr::Alphanumeric; // Ensure you import the correct Uniform type

use crate::util::Vec3;

#[derive(Clone)]
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
        self.random_float_range(0.0, 1.0)
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

    pub fn random_on_disk(&mut self) -> Vec3 {
        loop {
            let mut p = self.random_vec3_range(-1.0, 1.0);
            p.z = 0.0;
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_chars(&mut self, len: usize) -> String {
        Alphanumeric.sample_string(&mut self.rng, len)
    }
}