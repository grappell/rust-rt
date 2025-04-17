mod util;
mod hittable;
mod camera;
mod random;

use camera::Camera;
use util::Vec3;
use hittable::{HittableList, Sphere};
use random::RandomGenerator;

use Vec3 as Point;

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

    // Set up scene
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.3, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(-1.5, 0.0, -3.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
    
    // Render scene
    Camera::render(&mut camera, &world, &mut img_buf);

    // Save image
    img_buf.save(IMG_PATH).unwrap();
    println!("Image saved to {}", IMG_PATH);

    println!("Raytrace finished.");

}
