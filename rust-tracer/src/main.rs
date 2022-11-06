pub mod math;
pub mod primitives;
pub mod render;

use crate::math::{Vector3, Quadratic};
use crate::primitives::{Ray, Sphere, Intersectable, Intersect, PointLight};
use crate::render::{Camera, Scene, Renderer};

use rand::prelude::*;

extern crate image;

fn main() {

    // Test rays
    let v = Vector3::new(1.0, 2.0, 3.0);
    let u = Vector3 { x: 12.0, y: 13.0, z: -2.0 };
    let ray = Ray { origin: u, direction: v };
    let point = ray.get_point(15.0);
    println!("{} {} {}", point.x, point.y, point.z);

    // Test quadratic solve
    let result = Quadratic::solve(1.0, 2.1, -3.0);
    match result {
        Some(..) => println!("x1 = {}, x2 = {}", result.unwrap().0, result.unwrap().1),
        None => println!("Didn't find a solution!")
    };

    // Generate a set of random spheres.
    let mut intersectables = Vec::new();
    let n = 64;
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let pos = 1.3 * (Vector3::random() - Vector3::new(0.5, 0.5, 0.0));
        let r: f32 = rng.gen::<f32>() * 0.175 + 0.05;
        intersectables.push(Sphere::new(pos, r));
    }

    // Add a light
    let ambient = Vector3::new(0.04, 0.04, 0.04);
    let light_main = PointLight::new(Vector3::new(0.0, 2.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
    let light_back = PointLight::new(Vector3::new(0.0, 1.0, 7.5), Vector3::new(0.6, 0.6, 0.6));

    // Test rendering an image of a sphere.
    let camera = Camera::new(Vector3::new(0.0, 0.0, -5.0),
                             Vector3::new(0.0, 0.0, 0.0));

    let image_width = 1000;
    let image_height = 1000;

    let mut image_buffer = image::ImageBuffer::new(
        image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = (2.0 * x as f32 - image_width as f32) / (image_height as f32);
        let v = (2.0 * y as f32 - image_height as f32) / (image_height as f32);
        let mut min_dist = f32::MAX;
        let mut nearest_hit: Option<Intersect> = None;

        for intersectable in &intersectables {
            let ray = camera.get_ray(u, v);
            let hit = intersectable.intersect(&ray);

            match hit {
                Some(hit) => {
                    let distance = (*hit.object.position() - camera.position).length_squared();
                    if distance < min_dist {
                        min_dist = distance;
                        nearest_hit = Some(hit);
                    }
                }
                None => {
                    *pixel = image::Rgb([0u8 , 0u8, 0u8]);
                },
            };
        }

        match nearest_hit {
            Some(hit) => {
                let normal = hit.object.normal(&hit.position);

                let diffuse = light_main.ambient_color(&hit.position, &normal)
                            + light_back.ambient_color(&hit.position, &normal);
                            
                let color = ambient + diffuse;

                *pixel = image::Rgb([
                    (255.0 * color.x).clamp(0.0, 255.0) as u8, 
                    (255.0 * color.y).clamp(0.0, 255.0) as u8, 
                    (255.0 * color.z).clamp(0.0, 255.0) as u8]);
            },
            None => {
                *pixel = image::Rgb([0u8 , 0u8, 0u8]);
            }, 
        }
    }

    image_buffer.save("output.png").unwrap();
}
