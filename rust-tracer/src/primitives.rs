use crate::math::{Vector3, Quadratic};

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersect>;
}

pub trait Object3D<'a> {
    fn position(&self) -> &Vector3;
    fn normal(&self, point: &Vector3) -> Vector3;
}

pub struct Line {
    pub a: Vector3,
    pub b: Vector3,
}

impl Line {
    pub fn new(a: Vector3, b: Vector3) -> Line {
        Line { a, b }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
    pub fn get_point(&self, t: f32) -> Vector3 {
        self.origin + t * self.direction
    }
    pub fn reflect(&self, normal: &Vector3) -> Ray {
        let u_dot_n = self.direction.dot(normal);
        let direction = self.direction - 2.0 * (*normal) * u_dot_n;
        Ray::new(self.origin, direction)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub position: Vector3,
    pub radius: f32,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersect> {
        let l = ray.origin - self.position;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&l);
        let c = l.dot(&l) - self.radius * self.radius;
        match Quadratic::solve(a, b, c) {
            Some((mut t1, mut t2)) => {
                if t1 > t2 { (t1, t2) = (t2, t1); }
                let mut t = t1;
                if t < 0.0 { t = t2; }
                if t < 0.0 { return None }
                let hit = Intersect::new(t, 
                                         ray.get_point(t), 
                                         Box::new(*self));
                Some(hit)
            },
            None => None
        }
    }
}

impl Sphere {
    pub fn new(position: Vector3, radius: f32) -> Sphere {
        Sphere { position, radius }
    }
}

pub struct Plane {
    pub position: Vector3,
    pub u: Vector3,
    pub v: Vector3,
}

pub struct Intersect<'a> {
    pub t: f32,
    pub position: Vector3,
    pub object:  Box<dyn Object3D<'a>>
}

impl Intersect<'_> {
    pub fn new(t: f32, position: Vector3, object: Box<dyn Object3D>) -> Intersect {
        Intersect { t, position, object }
    }
}

pub struct PointLight { 
    pub position: Vector3,
    pub color: Vector3,
}

impl PointLight {
    pub fn new(position: Vector3, color: Vector3) -> PointLight {
        PointLight { position, color }
    }
    pub fn ambient_color(&self, p: &Vector3, n: &Vector3) -> Vector3 {
        let l = (*p - self.position).normalize();
        n.dot(&l) * self.color
    }
}

trait Clamp {
    fn clamp(self, a: u8, b: u8) -> u8;
}

impl Clamp for u8 {
    fn clamp(self, a: u8, b: u8) -> u8 {
        if self < a { a }
        else if self > b { b }
        else { self }
    }
}

impl Object3D<'_> for Sphere {
    fn position(&self) -> &Vector3 {
        &self.position
    }
    fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.position).normalize()
    }
}
