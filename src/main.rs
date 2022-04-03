use std::{fs::File, io::Write};

use rendering::{render_pixel, Color};

mod vector;
mod rendering;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut image_buffer: Vec<String> = vec![
        String::from("P3\n"),
        String::from("256"),
        String::from(" "),
        String::from("256"),
        String::from("\n255\n")
    ];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64/ (image_height - 1) as f64;

            let ir = 255.0 * r;
            let ig = 255.0 * g;
            let ib = 63.0;

            render_pixel(&mut image_buffer, Color { red: ir, green: ig, blue: ib });
        }
    }

    let mut file = File::create("test.ppm").unwrap();
    file.write_all(image_buffer.concat().as_bytes()).unwrap();
}
