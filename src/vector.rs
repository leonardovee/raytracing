use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.normalize().sqrt()
    }

    pub fn normalize(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(vec: &Vector3) -> Vector3 {
        Vector3 {
            x: vec.x / vec.length(),
            y: vec.y / vec.length(),
            z: vec.z / vec.length(),
        }
    }

    pub fn dot(vec1: &Vector3, vec2: &Vector3) -> f64 {
        (vec1.x * vec2.x) + (vec1.y * vec2.y) + (vec1.z * vec2.z)
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn unit_vector(v: &Vector3) -> Vector3 {
        *v / v.length()
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        Vector3::unit_vector(&Vector3::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let on_unit_sphere = Vector3::random_unit_vector();
        if Vector3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        let t = 1.0 / rhs;
        Vector3 {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z,
        }
    }
}

impl Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector3 {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_add() {
        let x = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let y = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let add = x + y;

        assert_eq!(add.x, 3.0);
        assert_eq!(add.y, 3.0);
        assert_eq!(add.z, 3.0);
    }

    #[test]
    fn test_vector3_sub() {
        let x = Vector3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        };
        let y = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let sub = x - y;

        assert_eq!(sub.x, 3.0);
        assert_eq!(sub.y, 3.0);
        assert_eq!(sub.z, 3.0);
    }

    #[test]
    fn test_vector3_mul() {
        let x = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let y = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }

    #[test]
    fn test_vector3_div() {
        let x = Vector3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        };
        let y = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let div = x / y;

        assert_eq!(div.x, 5.0);
        assert_eq!(div.y, 5.0);
        assert_eq!(div.z, 5.0);
    }

    #[test]
    fn test_vector3_length() {
        let x = Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        };

        let length = x.length();

        assert_eq!(length, 5.196152422706632);
    }

    #[test]
    fn test_vector3_unit() {
        let y = Vector3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        };

        let unit = Vector3::unit(&y);

        assert_eq!(
            unit,
            Vector3 {
                x: 0.5773502691896257,
                y: 0.5773502691896257,
                z: 0.5773502691896257
            }
        );
    }

    #[test]
    fn test_vector3_mul_by_i64() {
        let x = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let y = 2;

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }

    #[test]
    fn test_vector3_mul_by_f64() {
        let x = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let y = 2.0;

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }

    #[test]
    fn test_vector3_neg() {
        let x = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let neg = -x;

        assert_eq!(neg.x, -2.0);
        assert_eq!(neg.y, -2.0);
        assert_eq!(neg.z, -2.0);
    }

    #[test]
    fn test_vector3_scalar_multiplication() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;

        let mul = scalar * v;

        assert_eq!(mul.x, 2.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 6.0);
    }
}
