use std::{fs::File, io::Write};

use crate::{hittable_list::HittableList, ray::Ray, render::render_pixel, vector::Vector3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, image_height: f64) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            image_height,
        }
    }

    pub fn render(&self, world: HittableList) {
        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - Vector3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };
        let mut image_buffer: Vec<String> = vec![
            String::from("P3\n"),
            self.image_width.to_string(),
            String::from(" "),
            self.image_height.to_string(),
            String::from("\n255\n"),
        ];

        for j in (0..self.image_height as i64).rev() {
            for i in 0..self.image_width {
                let u = i as f64 / (self.image_width - 1) as f64;
                let v = j as f64 / (self.image_height - 1.0);

                let ray = Ray::new(
                    origin,
                    lower_left_corner + (horizontal * u) + (vertical * v) - origin,
                );
                let pixel_color = ray.color(&world);

                render_pixel(&mut image_buffer, pixel_color);
            }
        }

        let mut file = File::create("test.ppm").unwrap();
        file.write_all(image_buffer.concat().as_bytes()).unwrap();
    }
}
