use std::ops::{Add, Mul, Div, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        self.normalize().sqrt()
    }
    
    pub fn normalize(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit(vec: &Vector3) -> Vector3 {
        Vector3 {
            x: vec.x / vec.length(),
            y: vec.y / vec.length(),
            z: vec.z / vec.length(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_add() {
        let x = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
        let y = Vector3 { x: 2.0, y: 2.0, z: 2.0 };

        let add = x + y;

        assert_eq!(add.x, 3.0);
        assert_eq!(add.y, 3.0);
        assert_eq!(add.z, 3.0);
    }

    #[test]
    fn test_vector3_sub() {
        let x = Vector3 { x: 5.0, y: 5.0, z: 5.0 };
        let y = Vector3 { x: 2.0, y: 2.0, z: 2.0 };

        let sub = x - y;

        assert_eq!(sub.x, 3.0);
        assert_eq!(sub.y, 3.0);
        assert_eq!(sub.z, 3.0);
    }

    #[test]
    fn test_vector3_mul() {
        let x = Vector3 { x: 2.0, y: 2.0, z: 2.0 };
        let y = Vector3 { x: 2.0, y: 2.0, z: 2.0 };

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }

    #[test]
    fn test_vector3_div() {
        let x = Vector3 { x: 10.0, y: 10.0, z: 10.0 };
        let y = Vector3 { x: 2.0, y: 2.0, z: 2.0 };

        let div = x / y;

        assert_eq!(div.x, 5.0);
        assert_eq!(div.y, 5.0);
        assert_eq!(div.z, 5.0);
    }

    #[test]
    fn test_vector3_length() {
        let x = Vector3 { x: 3.0, y: 3.0, z: 3.0 };

        let length = x.length();

        assert_eq!(length, 5.196152422706632);
    }

    #[test]
    fn test_vector3_unit() {
        let y = Vector3 { x: 3.0, y: 3.0, z: 3.0 };

        let unit = Vector3::unit(&y);

        assert_eq!(unit, Vector3 { x: 0.5773502691896257, y: 0.5773502691896257, z: 0.5773502691896257 });
    }

    #[test]
    fn test_vector3_mul_by_i64() {
        let x = Vector3 { x: 2.0, y: 2.0, z: 2.0 };
        let y = 2;

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }

    #[test]
    fn test_vector3_mul_by_f64() {
        let x = Vector3 { x: 2.0, y: 2.0, z: 2.0 };
        let y = 2.0;

        let mul = x * y;

        assert_eq!(mul.x, 4.0);
        assert_eq!(mul.y, 4.0);
        assert_eq!(mul.z, 4.0);
    }
}
