use core::f32;
use std::sync::Arc;

use crate::util::{Ray, Vec3};
use crate::material::{Lambertian, Material};

use Vec3 as Point;
use Vec3 as Color;


#[derive(Debug)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { (*outward_normal) * -1.0 };
    }

    pub fn new() -> Self {
        HitRecord {
            point: Point::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Lambertian::new(Color::new(1.0, 1.0, 1.0)),
        }
    }

    pub fn clone(&self) -> Self {
        HitRecord {
            point: self.point,
            normal: self.normal,
            t: self.t,
            front_face: self.front_face,
            material: self.material.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Box<dyn Material>) -> Box<Self> {
    Box::new(Sphere { center, radius: radius.max(0.0), material })
    }
}

impl Hittable for Sphere {
    fn hit (&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {

        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (h - sqrt) / a;

        if !interval.surrounds(root) {
            root = (h + sqrt) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(root);
        rec.normal = (rec.point - self.center) / self.radius;
        rec.material = self.material.clone();

        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}


pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval, rec: &mut HitRecord) -> bool {
        
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in self.objects.iter() {

            if object.hit(r, Interval::new(interval.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
    
        }

        hit_anything

    }
}

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {

    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
       self.min < value && value < self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }

    pub fn empty() -> Interval {
        Interval { min: f32::MAX, max: f32::MIN }
    }

    pub fn universe() -> Interval {
        Interval { min: f32::MIN, max: f32::MAX }
    }
}