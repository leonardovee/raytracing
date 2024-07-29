use rand::Rng;
use std::{fs::File, io::Write};

use crate::{
    color::Color, hittable_list::HittableList, point::Point3, ray::Ray, render::render_pixel,
    vector::Vector3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            samples_per_pixel,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vector3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vector3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 0.0,
            max_depth,
        };
        camera.initialize();
        camera
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        self.center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    pub fn render(&self, world: &HittableList) {
        let mut image_buffer: Vec<String> = vec![format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )];

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + r.color(self.max_depth, world);
                }
                render_pixel(&mut image_buffer, pixel_color * self.pixel_samples_scale);
            }
        }

        let mut file = File::create("test.ppm").unwrap();
        file.write_all(image_buffer.concat().as_bytes()).unwrap();
    }
}
