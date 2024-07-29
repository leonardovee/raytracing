use crate::color::Color;
use crate::interval::Interval;
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

    pub fn color(&self, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(self, Interval::new(0.0, f64::INFINITY), &mut rec) {
            let direction = Vector3::random_on_hemisphere(&rec.normal);
            let new_ray = Ray::new(rec.p, direction);
            return new_ray.color(depth - 1, world) * 0.5;
        }

        let unit_direction = Vector3::unit(&self.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    struct MockHittable {
        should_hit: bool,
        normal: Vector3,
    }

    impl Hittable for MockHittable {
        fn hit(&self, _ray: &Ray, _int: Interval, rec: &mut HitRecord) -> bool {
            if self.should_hit {
                rec.normal = self.normal;
                rec.p = Point3::new(1.0, 1.0, 1.0);
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn test_ray_new() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(*ray.origin(), origin);
        assert_eq!(*ray.direction(), direction);
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
    fn test_ray_color_no_hit() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let world = MockHittable {
            should_hit: false,
            normal: Vector3::new(0.0, 0.0, 0.0),
        };
        let color = ray.color(1, &world);
        assert!((color.red - 0.5).abs() < EPSILON);
        assert!((color.green - 0.7).abs() < EPSILON);
        assert!((color.blue - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_color_hit() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let world = MockHittable {
            should_hit: true,
            normal: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = ray.color(1, &world);
        assert!(color.red >= 0.0 && color.red <= 0.5);
        assert!(color.green >= 0.0 && color.green <= 0.5);
        assert!(color.blue >= 0.0 && color.blue <= 0.5);
    }

    #[test]
    fn test_ray_color_max_depth() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let world = MockHittable {
            should_hit: true,
            normal: Vector3::new(0.0, 0.0, 1.0),
        };
        let color = ray.color(0, &world);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
}
