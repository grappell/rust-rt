mod util;
mod hittable;
mod camera;
mod random;
mod material;

use std::f32::consts::PI;

use camera::Camera;
use material::{Dielectric, Lambertian, Material, Metal};
use util::Vec3;
use hittable::{HittableList, Sphere};
use random::RandomGenerator;

use Vec3 as Point;
use Vec3 as Color;

// NOTES --------------------------
// y-axis go up, the x-axis to the right, and the negative z-axis pointing in the viewing direction

// System Constants
pub const IMG_PATH: &str = "res/image.png";

fn main() {

    println!(" \n Starting Code \n ");

    // Make random generator
    let cam_rand = RandomGenerator::new();
    let mut scene_rand = RandomGenerator::new();

    // Camera parameters
    let fov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);

    // Initialize camera
    let mut camera = Camera::initialize(fov, look_from, look_at, cam_rand);

    // Set up image buffer
    let mut img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32);

    // Set up scene
    let mut world = HittableList::new();

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, mat_ground));

    for a in -11..11 {
        for b in -22..11 {
            let choose_mat = &scene_rand.random_float();

            let center = Point::new(a as f32 + 0.9 * &scene_rand.random_float(), 0.2, b as f32 + 0.9 * &scene_rand.random_float());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if *choose_mat < 0.8 {
                    // diffuse
                    let albedo = &scene_rand.random_vec3_range(0.0, 1.0) * &scene_rand.random_vec3_range(0.0, 1.0);
                    let mat = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, mat));
                } else if *choose_mat < 0.95 {
                    // Metal
                    let albedo = &scene_rand.random_vec3_range(0.5, 1.0);
                    let fuzz = &scene_rand.random_float_range(0.0, 0.5);
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
    Camera::render(&mut camera, &world, &mut img_buf);

    // Save image
    img_buf.save(IMG_PATH).unwrap();
    println!("Image saved to {}", IMG_PATH);

    println!("Raytrace finished.");

}
