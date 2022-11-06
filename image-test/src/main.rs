// https://stackoverflow.com/questions/29836804/how-do-i-procedurally-generate-images-using-rust-image
// https://github.com/image-rs/image/blob/master/README.md
extern crate image;
extern crate rayon;
extern crate chrono;

use std::f32::consts::PI;

use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Complex {
    pub a: f32, 
    pub b: f32,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Complex { 
            a: self.a * rhs.a - self.b * rhs.b, 
            b: self.a * rhs.b + self.b * rhs.a ,
        }
    }
}

impl Complex {
    fn conj_sq(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

impl Complex {
    fn abs(self) -> Self {
        Complex { 
            a: self.a.abs(), 
            b: self.b.abs()
        }
    }
}

fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: 0.0, b: 0.0 };
    let c = Complex { a: x, b: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.conj_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f32 - z.conj_sq().log2().log2()) / (max as f32);
}

fn julia(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: x, b: y };
    let c = Complex { a: 0.38, b: 0.28 };
    let max = 256;
    let mut i = 0;
    while i < max && z.conj_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f32 - z.conj_sq().log2().log2()) / (max as f32);
}

fn burning_ship(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: 0.0, b: 0.0 };
    let c = Complex { a: x, b: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.conj_sq() < 32.0 {
        z = z.abs();
        z = z * z + c;
        i += 1;
    }
    return (i as f32 - z.conj_sq().log2().log2()) / (max as f32);
}

fn complex_sin(z: Complex) -> Complex {
    Complex { 
        a: z.a.sin() * z.b.cosh(),
        b: z.a.cos() * z.b.sinh()
    }
}

fn csinz(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: x, b: y };
    let c = Complex { a: -0.2, b: 1.0 };
    let max = 256;
    let mut i = 0;
    while i < max && z.b.abs() < 50.0 {
        z = c * complex_sin(z);
        i += 1;
    }
    return i as f32 / max as f32;
    // return (i as f32 - z.conj_sq().log2().log2()) / (max as f32) * 2.0 * PI - PI;
}

fn color(p: &Palette, t: f32) -> [u8; 3] {
    let r = p.b[0] * (6.28318 * (p.c[0] * (t % 1.0) + p.d[0])).cos() + p.a[0];
    let g = p.b[1] * (6.28318 * (p.c[1] * (t % 1.0) + p.d[1])).cos() + p.a[1];
    let b = p.b[2] * (6.28318 * (p.c[2] * (t % 1.0) + p.d[2])).cos() + p.a[2];
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}

struct Palette {
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    d: [f32; 3]
}

fn main() {
    let image_width = 4000;
    let image_height = 4000;
    let scale = 3.0;
    let eps = 1e-16;

    let mut image_buffer = image::ImageBuffer::new(
        image_width, image_height);

    let start_time = chrono::Utc::now();

    let pallete_brown_blue = Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [1.0, 1.0, 1.0],
        d: [0.0, 0.10, 0.20],
    };

    let pallete_purple_green_yellow = Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [2.0, 1.0, 0.0],
        d: [0.50, 0.20, 0.25],
    };

    let pallete_blue_red = Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [1.0, 1.0, 1.0],
        d: [0.30, 0.20, 0.20],
    };

    let palette_red_green = Palette {
        a: [0.8, 0.5, 0.4],
        b: [0.2, 0.4, 0.2],
        c: [2.0, 1.0, 1.0],
        d: [0.00, 0.25, 0.25],
    };

    let palette_green_yellow_red = Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [1.0, 1.0, 0.5],
        d: [0.80, 0.90, 0.30],
    };

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let u = x as f32 / image_height as f32;
        let v = y as f32 / image_height as f32;
        let aspect_ratio = image_width as f32 / image_height as f32;
        let t = csinz(scale * (u - 0.5 * aspect_ratio), scale * (v - 0.5));

        if t + eps < 1.0 {
            *pixel = image::Rgb(color(&palette_green_yellow_red, (2.0 * t + 0.85) % 1.0));
        }
        else {
            *pixel = image::Rgb([0, 0, 0]);
        }

        // image_buffer.put_pixel(x, y, pixel)
    }

    // (0..image_height).into_par_iter().map(|y| {
    //     (0..image_width).map(|x| {
    //         let u = x as f32 / image_height as f32;
    //         let v = y as f32 / image_height as f32;
    //         let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5) - 0.6);
    //         let color = image::Rgb(color((2.0 * t + 0.5) % 1.0));
    //         image_buffer.put_pixel(x, y, color);
    //     });
    // });

    let end_time = chrono::Utc::now();
    println!("Wall time: {} ms.", (end_time - start_time).num_milliseconds());

    image_buffer.save("render.png").unwrap();
}
