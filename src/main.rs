mod util;
mod hittable;
mod camera;
mod random;
mod material;

use camera::Camera;
use material::{Lambertian, Metal};
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

    // Initialize camera
    let mut camera = Camera::initialize(rand);

    // Set up image buffer
    let mut img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32);

    let material_ground = Box::new(Lambertian::new(Color::new(0.6, 0.6, 0.6)));
    let material_right = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.1, 0.2)));

    // Set up scene
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(1.1, 1.0, -4.0), 1.5, material_left)));
    world.add(Box::new(Sphere::new(Point::new(-1.0, 0.0, -2.0), 0.5, material_right)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -2.0), 100.0, material_ground)));
    
    // Render scene
    Camera::render(&mut camera, &world, &mut img_buf);

    // Save image
    img_buf.save(IMG_PATH).unwrap();
    println!("Image saved to {}", IMG_PATH);

    println!("Raytrace finished.");

}
