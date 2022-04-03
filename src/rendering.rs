use crate::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn color(&self) -> Color {
        Color { red: 1.0, green: 1.0, blue: 1.0 }
    }
}

pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn render_pixel(image_buffer: &mut Vec<String>, color: Color) {
    image_buffer.push(color.red.to_string());
    image_buffer.push(String::from(" "));

    image_buffer.push(color.green.to_string());
    image_buffer.push(String::from(" "));

    image_buffer.push(color.blue.to_string());
    image_buffer.push(String::from("\n"));
}
