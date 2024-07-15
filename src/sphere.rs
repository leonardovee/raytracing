use crate::{
    hittable::{HitRecord, Hittable},
    point::Point3,
    ray::Ray,
    vector::Vector3,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(point3: Point3, radius: f64) -> Self {
        Sphere {
            center: point3,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vector3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::vector::Vector3;
    use std::f64::EPSILON;

    #[test]
    fn test_sphere_new() {
        let center = Point3::new(1.0, 2.0, 3.0);
        let radius = 2.0;
        let sphere = Sphere::new(center, radius);

        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_sphere_hit_miss() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(1.0, 1.0, 0.0));
        let mut rec = HitRecord::new();

        assert!(!sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec));
    }

    #[test]
    fn test_sphere_hit_from_outside() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::new();

        assert!(sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec));
        assert!((rec.t - 4.0).abs() < EPSILON);
        assert_eq!(rec.p, Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal, Vector3::new(0.0, 0.0, -1.0));
        assert!(rec.front_face);
    }

    #[test]
    fn test_sphere_hit_from_inside() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::new();

        assert!(sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec));
        assert!((rec.t - 1.0).abs() < EPSILON);
        assert_eq!(rec.p, Point3::new(0.0, 0.0, 1.0));
        assert_eq!(rec.normal, Vector3::new(0.0, 0.0, -1.0));
        assert!(!rec.front_face);
    }

    #[test]
    fn test_sphere_hit_beyond_tmax() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::new();

        assert!(!sphere.hit(&ray, 0.0, 3.0, &mut rec));
    }

    #[test]
    fn test_sphere_hit_before_tmin() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -0.5), Vector3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::new();

        assert!(!sphere.hit(&ray, 2.0, f64::INFINITY, &mut rec));
    }
}
