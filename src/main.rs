mod util;
mod hittable;
mod camera;
mod random;
mod material;

use std::f32::consts::PI;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
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
    let rand = RandomGenerator::new();

    // Camera parameters
    let fov = 20.0;
    let look_from = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);

    // Initialize camera
    let mut camera = Camera::initialize(fov, look_from, look_at, rand);

    // Set up image buffer
    let mut img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32);

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(Color::new(0.5, 0.5, 0.5), 1.33);
    let mat_bubble = Dielectric::new(Color::new(0.2, 0.1, 0.1), 1.0/1.33);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);


    // Set up scene
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, mat_ground));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, mat_center));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.1, mat_bubble));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, mat_right));
    
    // Render scene
    Camera::render(&mut camera, &world, &mut img_buf);

    // Save image
    img_buf.save(IMG_PATH).unwrap();
    println!("Image saved to {}", IMG_PATH);

    println!("Raytrace finished.");

}
