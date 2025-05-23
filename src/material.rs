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
    pub fn new(color: Color) -> Box<Self> {
        Box::new(Lambertian { albedo: color })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.normal + rand.random_unit_vector_on_sphere();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_dir);
        let attenuation = self.albedo;

        Option::Some((scattered, attenuation))
    }

    fn clone(&self) -> Box<dyn Material> {
        Lambertian::new(self.albedo)
    }
}

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
    specular: f32,
}

impl Metal {
    pub fn new(color: Color, fuzz: f32) -> Box<Self> {
        Box::new(Metal { albedo: color , specular: fuzz.max(0.0) })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)> {

        let mut reflected = Vec3::reflect(&ray_in.direction(), &rec.normal);
        reflected = reflected.unit_vector() + (rand.random_unit_vector_on_sphere() * self.specular);
        let scattered = Ray::new(rec.point, reflected);

        match Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            true => Option::Some((scattered, self.albedo)),
            false => Option::None,
        }

    }

    fn clone(&self) -> Box<dyn Material> {
       Metal::new(self.albedo, self.specular)
    }
}

#[derive(Clone, Debug)]
pub struct Dielectric {
    albedo: Color,
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(color: Color, refractive_index: f32) -> Box<Self> {
        Box::new(Dielectric { albedo: color , refractive_index })
    }

    fn reflectance(&self, cosine: f32, refractive_index: f32) -> f32 {
        let r0 = ((1.0 - refractive_index) / (1.0 + self.refractive_index)).powi(2);
        r0 + (1.0 - r0 ) * ((1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rand: &mut RandomGenerator) -> Option<(Ray, Vec3)> {

        let ri = if rec.front_face { 1.0 / self.refractive_index } else { self.refractive_index };
            
        let unit_direction = ray_in.direction().unit_vector() * -1.0;

        let cos_theta = Vec3::dot(&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let dir = if cannot_refract || self.reflectance(cos_theta, ri) > rand.random_float_range(0.0, 1.0)
            {Vec3::reflect(&unit_direction, &rec.normal)} else 
            {Vec3::refract(&unit_direction, &rec.normal, ri)};

        Some((Ray::new(rec.point, dir), Color::new(1.0, 1.0, 1.0))) // TODO: Fix this to use the albedo
        
    }

    fn clone(&self) -> Box<dyn Material> {
        Dielectric::new(self.albedo, self.refractive_index)
    }

}