use std::{fs::File, io::Write};

use rendering::{render_pixel, Ray};
use vector::Vector3;

mod vector;
mod rendering;

fn main() {
    // image definitions
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    // camera definition
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vector3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vector3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3 { x: 0.0, y: 0.0, z: focal_length };

    // render process
    let mut image_buffer: Vec<String> = vec![
        String::from("P3\n"),
        image_width.to_string(),
        String::from(" "),
        image_height.to_string(),
        String::from("\n255\n")
    ];

    for j in (0..image_height as i64).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1.0);

            let ray = Ray { origin, direction: lower_left_corner + (horizontal * u) + (vertical * v) - origin};
            let pixel_color = ray.color();

            render_pixel(&mut image_buffer, pixel_color);
        }
    }

    let mut file = File::create("test.ppm").unwrap();
    file.write_all(image_buffer.concat().as_bytes()).unwrap();
}
