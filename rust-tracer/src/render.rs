use crate::math::{Vector3};
use crate::primitives::*;

pub struct Camera {
    pub position: Vector3,
    pub direction: Vector3,
}

impl Camera {
    pub fn new(position: Vector3, direction: Vector3) -> Camera {
        Camera { position, direction }
    }
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = (Vector3::new(x, y, 0.0) - self.position).normalize();
        Ray::new(self.position, rd)
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
}

pub struct Renderer<'a> {
    pub scene: &'a Scene,
    pub camera: &'a Camera,
}
