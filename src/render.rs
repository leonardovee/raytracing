use crate::color::Color;

pub fn render_pixel(image_buffer: &mut Vec<String>, color: Color) {
    let r = (255.999 * color.red).clamp(0.0, 255.0) as u8;
    let g = (255.999 * color.green).clamp(0.0, 255.0) as u8;
    let b = (255.999 * color.blue).clamp(0.0, 255.0) as u8;

    image_buffer.push(format!("{} {} {}\n", r, g, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pixel_white() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "255 255 255\n");
    }

    #[test]
    fn test_render_pixel_black() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "0 0 0\n");
    }

    #[test]
    fn test_render_pixel_red() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "255 0 0\n");
    }

    #[test]
    fn test_render_pixel_fractional() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 0.5,
            green: 0.25,
            blue: 0.75,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "127 63 191\n");
    }

    #[test]
    fn test_render_pixel_rounding() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 0.999,
            green: 0.001,
            blue: 0.5,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "255 0 127\n");
    }
}
