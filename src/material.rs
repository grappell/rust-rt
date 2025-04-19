use crate::hittable::HitRecord;
use crate::util::{Ray, Vec3};
use crate::random::RandomGenerator;

use std::fmt::Debug;

use Vec3 as Color;

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)>;
    fn clone(&self) -> Box<dyn Material>;
}


#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.normal + rand.random_unit_vector_on_sphere();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_dir);
        let attenuation = self.albedo;

        Option::Some((scattered, attenuation))
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&ray_in.direction(), &rec.normal);
        Option::Some((Ray::new(rec.point, reflected), self.albedo))
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo))
    }
}