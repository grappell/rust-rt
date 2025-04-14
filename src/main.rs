mod util;

use util::{Ray, Vec3};

use Vec3 as Point;
use Vec3 as Color;

// NOTES --------------------------
// y-axis go up, the x-axis to the right, and the negative z-axis pointing in the viewing direction

fn main() {

    println!(" \n Starting Code \n ");

    let v1 = Point::new(1.0, 2.0, 3.0);
    println!("v1: {:?}", v1);

    // System Constants
    const IMG_PATH: &str = "res/image.png";

    // Image Constants
    const ASPECT_RATIO: f32 = 16.0 / 9.0; // Ideal aspect ratio
    const IMG_WIDTH: u32 = 1920;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO).max(1.0) as u32;

    // Viewport Constants
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMG_WIDTH / IMG_HEIGHT) as f32;

    // Camera Constants
    const FOCAL_LENGTH: f32 = 1.0;
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);

    // Vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / IMG_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMG_HEIGHT as f32;

    // Location of upper left pixel
    let viewport_top_left = camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let first_pixel_loc = viewport_top_left + (( pixel_delta_u / 2.0 + pixel_delta_v / 2.0) * 0.5 as f32);


    println!(" \n Running Raytrace... \n ");


    let mut img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);
    let mut prev_line = 0;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {

        if y != prev_line {
            println!("Rendering line {} of {} ({}%)", y, IMG_HEIGHT, y * 100 / IMG_HEIGHT);
            prev_line = y;
        }

        let c = Color::new(
            x as f32 / IMG_WIDTH as f32,
            y as f32 / IMG_HEIGHT as f32,
            1.0 - ((y + x) as f32 / (IMG_WIDTH + IMG_HEIGHT) as f32),
        );

        write_color(pixel, c);

    }

    println!("\n --- Done rendering --- \n");
    img_buf.save(IMG_PATH).unwrap();
    println!("Image saved to {}", IMG_PATH);
    println!("Raytrace finished.");

}

fn ray_color(ray: &Ray) -> Color {
    Color::new(0.0, 0.0, 0.0)
}

fn write_color(
    pixel: &mut image::Rgb<u8>,
    color: Color,
) {
    let r = (color.x * 255.0) as u8;
    let g = (color.y * 255.0) as u8;
    let b = (color.z * 255.0) as u8;

    *pixel = image::Rgb([r, g, b]);
}