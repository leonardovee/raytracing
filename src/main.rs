use std::time::Instant;

use hittable_list::HittableList;
use point::Point3;
use ray::Ray;
use sphere::Sphere;

use crate::camera::Camera;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod point;
mod ray;
mod render;
mod sphere;
mod vector;

fn main() {
    // image definitions
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = image_width as f64 / aspect_ratio;

    println!(
        "Rendering image with resolution: {}x{}",
        image_width, image_height
    );

    let start_time = Instant::now();

    // world setup
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // render
    let camera = Camera::new(aspect_ratio, image_width, 100);
    camera.render(&world);

    let elapsed_time = start_time.elapsed();
    println!("Rendering completed in {:.2?}", elapsed_time);
}
