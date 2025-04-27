mod util;
mod hittable;
mod camera;
mod random;
mod material;

use std::sync::{Arc, Mutex};

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use util::Vec3;
use hittable::{HittableList, Sphere};
use random::RandomGenerator;

use Vec3 as Point;
use Vec3 as Color;

// NOTES --------------------------
// y-axis go up, the x-axis to the right, and the negative z-axis pointing in the viewing direction

fn main() {

    println!(" \n Starting Code \n ");

    // Make random generator
    let mut rand = RandomGenerator::new();

    let mut img_path = "res/image".to_owned();
    img_path.push_str(&rand.random_chars(4));
    img_path.push_str(".png");

    // Camera parameters
    let fov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);

    // Initialize camera
    let camera = Camera::initialize(fov, look_from, look_at);

    // Set up image buffer
    // let img_buf = Arc::new(Mutex::new(image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32)));
    let img_buf = image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32);

    // Set up scene
    let mut world = HittableList::new();

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, mat_ground));

    for a in -22..22{
        for b in -11..11 {
            let choose_mat = &rand.random_float();

            let center = Point::new(a as f32 + 0.9 * &rand.random_float(), 0.2, b as f32 + 0.9 * &rand.random_float());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if *choose_mat < 0.8 {
                    // diffuse
                    let albedo = &rand.random_vec3_range(0.0, 1.0) * &rand.random_vec3_range(0.0, 1.0);
                    let mat = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, mat));
                } else if *choose_mat < 0.95 {
                    // Metal
                    let albedo = &rand.random_vec3_range(0.5, 1.0);
                    let fuzz = &rand.random_float_range(0.0, 0.5);
                    let mat = Metal::new(*albedo, *fuzz);
                    world.add(Sphere::new(center, 0.2, mat));
                } else {
                    // Glass
                    let mat = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
                    world.add(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }

    let mat_1 = Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5);
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat_1));

    let mat_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat_2));

    let mat_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat_3));
    
    // Render scene
    Camera::render(Arc::new(camera), Arc::new(world), img_buf, img_path);

    println!("Raytrace finished.");

}
