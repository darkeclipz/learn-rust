pub mod vector;
pub mod point;
use vector::Vector3;
use point::Point;
use image::{DynamicImage};


pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0
            }
            .normalize(),
        }
    }

    pub fn zero() -> Ray {
        Ray {
            origin: Point::zero(),
            direction: Vector3::zero()
        }
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let l: Vector3 = self.center - ray.origin;
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 300,
        height: 300,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4
            }
        }
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn main() {
    println!("Hello, world!");

    let s = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -5.0
        },
        radius: 1.0,
        color: Color {
            red: 0.4,
            green: 1.0,
            blue: 0.4
        }
    };

    let r = Ray::zero();

    s.intersect(&r);
}
