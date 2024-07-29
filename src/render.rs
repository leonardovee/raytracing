use crate::color::Color;
use crate::interval::Interval;

pub fn render_pixel(image_buffer: &mut Vec<String>, color: Color) {
    static INTENSITY: Interval = Interval::new(0.000, 0.999);

    let r = (256.0 * INTENSITY.clamp(color.red)) as i32;
    let g = (256.0 * INTENSITY.clamp(color.green)) as i32;
    let b = (256.0 * INTENSITY.clamp(color.blue)) as i32;

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
        assert_eq!(result, "128 64 192\n");
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
        assert_eq!(result, "255 0 128\n");
    }

    #[test]
    fn test_render_pixel_clamping() {
        let mut buffer = Vec::new();
        let color = Color {
            red: 1.1,
            green: -0.1,
            blue: 2.0,
        };
        render_pixel(&mut buffer, color);

        let result = buffer.join("");
        assert_eq!(result, "255 0 255\n");
    }
}
