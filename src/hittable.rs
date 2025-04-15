use crate::util::{Ray, Vec3};

use Vec3 as Point;
use Vec3 as Color;

struct Hit_record {
    point: Point,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl Hit_record {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { (*outward_normal) * -1.0 };
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Hit_record) -> bool;
}

struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    fn new(center: Point, radius: f32) -> Self {
        Sphere { center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit (&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Hit_record) -> bool {

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

        if root <= t_min || root >= t_max {
            root = (h + sqrt) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.point = r.at(root);
        rec.normal = (rec.point - self.center) / self.radius;

        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}


struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}