use crate::random::RandomGenerator;
use crate::util::{linear_to_gamma, Ray, Vec3};
use crate::hittable::{HitRecord, Hittable, HittableList, Interval};
use image::ImageBuffer;
use Vec3 as Point;
use Vec3 as Color;

use std::sync::{mpsc, Arc};
use std::time::SystemTime;
use std::{thread, vec};

pub struct Camera {
   
    pub max_depth: u32, // Maximum depth of recursion for ray tracing

    // Image Constants
    pub img_width: u32,
    pub img_height: u32,

    pub samples_per_pixel: u32,
    pub pixel_sample_scale: f32,

    // Camera Constants
    pub camera_center: Point,

    // Horizontal and vertical delta vectors from pixel to pixel
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,

    // Location of upper left pixel
    pub first_pixel_loc: Vec3,

    pub defocus_angle: f32, // Variation angle of rays through each pixel

    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,

}

impl Camera {

    pub fn initialize(fov: f32, look_from: Point, look_at: Point) -> Camera {

        let max_depth: u32 = 10; // Maximum depth of recursion for ray tracing
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let defocus_angle = 3.0;
        let focus_dist = 10.0;

        // Image Constants
        let aspect_ratio: f32 = 16.0 / 9.0; // Ideal aspect ratio
        let img_width: u32 = 512;
        let img_height: u32 = (img_width as f32 / aspect_ratio).max(1.0) as u32;

        let samples_per_pixel: u32 = 50; // Number of samples per pixel
        let pixel_sample_scale: f32 = 1.0 / samples_per_pixel as f32; // Scale for averaging pixel samples

        // Camera Constants
        let camera_center: Point = look_from;

        // Viewport Constants
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width: f32 = viewport_height * (img_width as f32 / img_height as f32);

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / img_width as f32;
        let pixel_delta_v = viewport_v / img_height as f32;

        // Location of upper left pixel
        let viewport_top_left = camera_center - (w * focus_dist) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let first_pixel_loc = viewport_top_left + (( pixel_delta_u / 2.0 + pixel_delta_v / 2.0) * 0.5 as f32);

        let defocus_rad = focus_dist as f32 * (defocus_angle as f32 / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_rad;
        let defocus_disk_v = v * defocus_rad;
        // let rand = Arc::new(UnsafeCell::new(RandomGenerator::new()));

        Camera {
            max_depth,
            img_width,
            img_height,
            samples_per_pixel,
            pixel_sample_scale,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            first_pixel_loc,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v
        }
    }


    pub fn render(camera: Arc<Camera>, world: Arc<HittableList>, mut img_buf: ImageBuffer<image::Rgb<u8>, Vec<u8>>, img_path: String) {
        
        println!("\nRunning Parallel Raytrace... \n");
        let now = SystemTime::now();

        let num_cores = thread::available_parallelism().unwrap().get() as u32;
        // let rows_per_thread = (camera.img_height as f32 / num_cores as f32).ceil() as u32;
        let rows_per_thread = camera.img_height / num_cores;
        let mut rcv_count = 0;

        println!("\nUsing {} cores\n", num_cores);

        let (tx, rx) = mpsc::channel();

        let mut handles = vec![];

        for thread in 0..num_cores {

            let camera = Arc::clone(&camera);
            let world = Arc::clone(&world);
            let tx = tx.clone();

            let handle = thread::spawn(move || {

                let mut rand = RandomGenerator::new();
                let start_row = thread * rows_per_thread as u32;
                let end_row = (thread + 1) * rows_per_thread as u32;

                let mut section = vec![];

                for y in start_row..end_row {

                    println!("[THREAD {}] Rendering {}% done (line {} of {})", thread, (((y - start_row) as f32 / (end_row - start_row) as f32) * 100.0).round(), y - start_row, end_row - start_row);

                    for x in 0..camera.img_width {
    
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        
                        for _sample in 0..camera.samples_per_pixel {
                            let r = camera.get_ray(x as f32, y as f32, &mut rand);
                            pixel_color = pixel_color + camera.ray_color(&r, &world, camera.max_depth, &mut rand);
                        }
    
                        section.push(PixelData {
                            loc: (x, y),
                            color: pixel_color * camera.pixel_sample_scale
                        });

                    }
                }

                println!("\n [THREAD {}] FINISHED - Sending Section \n", thread);
                tx.send(section).unwrap();
    
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
            // println!("[THREAD] Rendering: {}% done ({} of {})", ((threads_completed as f32 / num_cores as f32) * 100.0).round(), threads_completed, num_cores);
        }

        for section in rx.iter() {
            
            for px in section {
                Camera::write_color(img_buf.get_pixel_mut(px.loc.0, px.loc.1), px.color);
            }

            rcv_count += 1;

            println!("[MAIN] Writing Section: {}% done ({} of {})", ((rcv_count as f32 / num_cores as f32) * 100.0).round(), rcv_count, num_cores);

            if rcv_count == num_cores as i32 {
                break;
            }

        }
    
        img_buf.save(&img_path).expect(("Unable to save image to ".to_owned() + &img_path).as_str());
        println!("\nImage saved to {}", &img_path);

        let after = SystemTime::now();
        let duration = after.duration_since(now).expect("Clock went backwards??");

        println!("\nRender Stats: \n - Total render time: {} sec \n - Total Pixels Calculated: {} \n - Average px/ms: {} \n", &duration.as_secs(), camera.img_width * camera.img_height, (camera.img_width * camera.img_height) as u128 / &duration.as_millis());
        
    }

    fn get_ray(&self, i: f32, j: f32, rand: &mut RandomGenerator) -> Ray {

        let offset = rand.random_vec3_square();
        let pixel_sample = self.first_pixel_loc
            + (self.pixel_delta_u * (i + offset.x))
            + (self.pixel_delta_v * (j + offset.y));

        let origin = if self.defocus_angle <= 0.0 { self.camera_center } else {self.defocus_disk_sample(rand)};
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self, rand: &mut RandomGenerator) -> Vec3 {
        let p = rand.random_on_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList, depth: u32, rand: &mut RandomGenerator) -> Color {

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0); // No more light is collected if at max depth
        }

        let rec = &mut HitRecord::new();
        if world.hit(ray, Interval::new(0.001, f32::MAX), rec) {
            let res = rec.material.scatter(ray, rec, rand);
            if res.is_some() {
                let (scattered, attenuation) = res.unwrap();
                return &self.ray_color(&scattered, world, depth - 1, rand) * &attenuation;
            }
            return Color::new(0.0, 0.0, 0.0); // No light is collected if no scatter occurs
        }
    
        // Background color
        let unit_vec = Vec3::unit_vector(ray.direction());
        let a = 0.8 * (unit_vec.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5 - 0.1, 0.7 - 0.1, 1.0) * a
    }
    
    fn write_color(pixel: &mut image::Rgb<u8>, color: Color) {
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
        // image::Rgb([rbyte, gbyte, bbyte])
    }
}

#[derive(Debug)]
pub struct PixelData {
    loc: (u32, u32),
    color: Color,
}