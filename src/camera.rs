use crate::util::{Ray, Vec3};
use crate::hittable::{HitRecord, Hittable, HittableList, Interval, Sphere};
use Vec3 as Point;
use Vec3 as Color;

// System Constants
pub const IMG_PATH: &str = "res/image.png";

pub struct Camera {

    // Image Constants
    pub aspect_ratio: f32, // Ideal aspect ratio
    pub img_width: u32,
    pub img_height: u32,

    // Viewport Constants
    pub viewport_height: f32,
    pub viewport_width: f32,

    // Camera Constants
    pub focal_length: f32,

    pub camera_center: Point,

    // Vectors across the horizontal and down the vertical viewport edges
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,

    // Horizontal and vertical delta vectors from pixel to pixel
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,

    // Location of upper left pixel
    pub viewport_top_left: Vec3,
    pub first_pixel_loc: Vec3,

}

impl Camera {

    pub fn initialize() -> Camera {
        // Image Constants
        let aspect_ratio: f32 = 16.0 / 9.0; // Ideal aspect ratio
        let img_width: u32 = 1000;
        let img_height: u32 = (img_width as f32 / aspect_ratio).max(1.0) as u32;

        // Viewport Constants
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (img_width as f32 / img_height as f32);

        // Camera Constants
        let focal_length: f32 = 1.0;
        let camera_center: Point = Point::new(0.0, 0.0, 0.0);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        // Location of upper left pixel
        let viewport_top_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let first_pixel_loc = viewport_top_left + (( pixel_delta_u / 2.0 + pixel_delta_v / 2.0) * 0.5 as f32);

        Camera {
            aspect_ratio,
            img_width,
            img_height,
            viewport_height,
            viewport_width,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_top_left,
            first_pixel_loc,
        }
    }


    pub fn render(world: &HittableList) {

        let camera = Camera::initialize();

        println!(" \n Running Raytrace... \n ");

        let mut img_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(camera.img_width as u32, camera.img_height as u32);
        let mut prev_line = 0;
    
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    
            // y --> i, x --> j
    
            if y != prev_line {
                println!("Rendering line {} of {} ({}%)", y, camera.img_height, y * 100 / camera.img_height + 1);
                prev_line = y;
            }
    
            // Calculate ray parameters
            let pixel_center = camera.first_pixel_loc + (camera.pixel_delta_u * x as f32) + (camera.pixel_delta_v * y as f32);
            let ray_dir = pixel_center - camera.camera_center;
            let ray = Ray::new(camera.camera_center, ray_dir);
    
            // Calculate the color of the pixel by tracing the ray
            let color = Camera::ray_color(&ray, &world);
            // Write the color to the pixel
            Camera::write_color(pixel, color);
    
        }
    
        println!("\n --- Done rendering --- \n");
        img_buf.save(IMG_PATH).unwrap();
        println!("Image saved to {}", IMG_PATH);
    
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {

        let rec = &mut HitRecord::new();
        if world.hit(ray, Interval::new(0.0, f32::MAX), rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }
    
        // Background color
        let unit_vec = Vec3::unit_vector(ray.direction());
        let a = 0.8 * (unit_vec.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
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
}