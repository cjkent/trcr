use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mag(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalised(&self) -> Vec3 {
        let mag = self.mag();
        Vec3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f64) -> Vec3 {
        Vec3 { x: self.x / divisor, y: self.y / divisor, z: self.z / divisor }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, multiplier: f64) -> Vec3 {
        Vec3 { x: self.x * multiplier, y: self.y * multiplier, z: self.z * multiplier }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        println!("hello, world");
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec * 2.0, Vec3::new(2.0, 4.0, 6.0));
        println!("{:?}", vec);
    }

    #[test]
    fn normalised() {
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).normalised(), Vec3::new(1.0, 0.0, 0.0))
    }
}
