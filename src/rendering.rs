use std::{ops::{Mul, Add}};

use crate::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn color(&self) -> Color {
        let unit_direction = Vector3::unit(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (Color { red: 1.0, green: 1.0, blue: 1.0 } * (1.0 - t)) + (Color { red: 0.5, green: 0.7, blue: 1.0 } * t)
    }
}

#[derive(Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

pub fn render_pixel(image_buffer: &mut Vec<String>, color: Color) {
    image_buffer.push((255.999 * color.red).to_string());
    image_buffer.push(String::from(" "));

    image_buffer.push((255.999 * color.green).to_string());
    image_buffer.push(String::from(" "));

    image_buffer.push((255.999 * color.blue).to_string());
    image_buffer.push(String::from("\n"));
}
