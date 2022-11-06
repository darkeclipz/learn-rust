use std::ops::{Add, Sub, Mul};
use rand::prelude::*;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn normalize(&self) -> Vector3 {
        let d = self.length();
        Self::new(self.x / d, self.y / d, self.z / d)
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn random() -> Vector3 {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }  
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self: Vector3, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        rhs * self
    }
}

pub struct Quadratic;

impl Quadratic {
    pub fn solve(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
        let discr = b * b - 4.0 * a * c;
        if discr < 0.0 { 
            None 
        }
        else if discr == 0.0 { 
            let x1 = -0.5 * b / a;
            Some((x1, x1))
        }
        else {
            let q = if b > 0.0 {
                -0.5 * (b + discr.sqrt())
            }
            else {
                -0.5 * (b - discr.sqrt())
            };
            let x0 = q / a;
            let x1 = c / q;
            Some(Self::sort(x0, x1))
        }
    }

    fn sort(a: f32, b: f32) -> (f32, f32) {
        if a < b { (a, b) } else { (b, a) }
    }
}