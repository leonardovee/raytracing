use crate::color::Color;
use crate::{
    hittable::{HitRecord, Hittable},
    point::Point3,
    vector::Vector3,
};

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.direction * t)
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(&self, 0.0, f64::INFINITY, &mut rec) {
            let color_vector = 0.5 * (rec.normal + Vector3::new(1.0, 1.0, 1.0));
            return Color {
                red: color_vector.x,
                green: color_vector.y,
                blue: color_vector.z,
            };
        }

        let unit_direction = Vector3::unit(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        } * (1.0 - t)
            + Color {
                red: 0.5,
                green: 0.7,
                blue: 1.0,
            } * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point3;
    use crate::vector::Vector3;
    use std::f64::EPSILON;

    #[test]
    fn test_ray_new() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        let point = ray.at(2.0);
        assert_eq!(point, Vector3::new(9.0, 12.0, 15.0));
    }

    #[test]
    fn test_ray_origin() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin(), &origin);
    }

    #[test]
    fn test_ray_direction() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.direction(), &direction);
    }

    #[test]
    fn test_ray_color_hit() {
        struct MockHittable;
        impl Hittable for MockHittable {
            fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, rec: &mut HitRecord) -> bool {
                rec.normal = Vector3::new(0.0, 1.0, 0.0);
                true
            }
        }

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let world = MockHittable;
        let color = ray.color(&world);

        assert!((color.red - 0.5).abs() < EPSILON);
        assert!((color.green - 1.0).abs() < EPSILON);
        assert!((color.blue - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_ray_color_miss() {
        struct MockHittable;
        impl Hittable for MockHittable {
            fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
                false
            }
        }

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let world = MockHittable;
        let color = ray.color(&world);

        let unit_direction = Vector3::unit(&ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        let expected_red = (1.0 * (1.0 - t)) + (0.5 * t);
        let expected_green = (1.0 * (1.0 - t)) + (0.7 * t);
        let expected_blue = (1.0 * (1.0 - t)) + (1.0 * t);

        assert!((color.red - expected_red).abs() < EPSILON);
        assert!((color.green - expected_green).abs() < EPSILON);
        assert!((color.blue - expected_blue).abs() < EPSILON);
    }
}
