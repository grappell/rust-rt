use crate::random::RandomGenerator;
use crate::util::{linear_to_gamma, Ray, Vec3};
use crate::hittable::{HitRecord, Hittable, HittableList, Interval};
use image::{ImageBuffer, Rgb};
use Vec3 as Point;
use Vec3 as Color;

// System Constants
pub const IMG_PATH: &str = "res/image.png";

pub struct Camera {

    pub max_depth: u32, // Maximum depth of recursion for ray tracing

    // Image Constants
    pub aspect_ratio: f32, // Ideal aspect ratio
    pub img_width: u32,
    pub img_height: u32,

    pub samples_per_pixel: u32,
    pub pixel_sample_scale: f32,

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

    pub rand: RandomGenerator,

}

impl Camera {

    pub fn initialize(rand: RandomGenerator) -> Camera {

        let max_depth: u32 = 10; // Maximum depth of recursion for ray tracing

        // Image Constants
        let aspect_ratio: f32 = 16.0 / 9.0; // Ideal aspect ratio
        let img_width: u32 = 512;
        let img_height: u32 = (img_width as f32 / aspect_ratio).max(1.0) as u32;

        let samples_per_pixel: u32 = 100; // Number of samples per pixel
        let pixel_sample_scale: f32 = 1.0 / samples_per_pixel as f32; // Scale for averaging pixel samples

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
            max_depth,
            aspect_ratio,
            img_width,
            img_height,
            samples_per_pixel,
            pixel_sample_scale,
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
            rand
        }
    }


    pub fn render(camera: &mut Camera, world: &HittableList, img_buf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {

        println!(" \n Running Raytrace... \n ");

        let mut prev_line = 0;
    
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
    
            // y --> i, x --> j
    
            if y != prev_line {
                println!("Rendering line {} of {} ({}%)", y, camera.img_height, y * 100 / camera.img_height + 1);
                prev_line = y;
            }

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // Sample the pixel multiple times for anti-aliasing
            for _sample in 0..camera.samples_per_pixel {
                let r = camera.get_ray(x as f32, y as f32);
                pixel_color = pixel_color + camera.ray_color(&r, world, camera.max_depth);
            }
        
            // Write the color to the pixel
            Camera::write_color(pixel, pixel_color * camera.pixel_sample_scale);
    
        }
    
        println!("\n --- Done rendering --- \n");
    
    }

    fn get_ray(&mut self, i: f32, j: f32) -> Ray {

        let offset = self.rand.random_vec3_square();
        let pixel_sample = self.first_pixel_loc
            + (self.pixel_delta_u * (i + offset.x))
            + (self.pixel_delta_v * (j + offset.y));

        let origin = self.camera_center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn ray_color(&mut self, ray: &Ray, world: &HittableList, depth: u32) -> Color {

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0); // No more light is collected if at max depth
        }

        let rec = &mut HitRecord::new();
        if world.hit(ray, Interval::new(0.001, f32::MAX), rec) {
            let res = rec.material.scatter(ray, rec, &mut self.rand);
            if res.is_some() {
                let (scattered, attenuation) = res.unwrap();
                return &self.ray_color(&scattered, world, depth - 1) * &attenuation;
            }
            return Color::new(0.0, 0.0, 0.0); // No light is collected if no scatter occurs
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
        let mut r = color.x;
        let mut g = color.y;
        let mut b = color.z;

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        static INTENSITY: Interval = Interval::new(0.0, 0.99999);
        let rbyte = (256.0 * INTENSITY.clamp(r as f32)) as u8;
        let gbyte = (256.0 * INTENSITY.clamp(g as f32)) as u8;
        let bbyte = (256.0 * INTENSITY.clamp(b as f32)) as u8;
    
        *pixel = image::Rgb([rbyte, gbyte, bbyte]);
    }
}