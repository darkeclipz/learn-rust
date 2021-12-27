// https://stackoverflow.com/questions/29836804/how-do-i-procedurally-generate-images-using-rust-image
// https://github.com/image-rs/image/blob/master/README.md
extern crate image;

fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = (0f32, 0f32);
    let c = (x, y);
    let max_iterations = 64;
    let mut i = 0;
    while i < max_iterations {
        z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
        if z.0 * z.0 + z.1 * z.1 > 4.0 {
            // Smooth iteration count:  (i - log2(log2(dot(z, z)))) / n  
            let norm = z.0 * z.0 + z.1 * z.1;
            return (i as f32 - norm.log2().log2()) / max_iterations as f32;
        }
        i += 1;
    }
    1.0
}

///! An example of writing images!
fn main() {

    let image_width = 800;
    let image_height = 500;
    let scale = 2.5;
    let translate_x = -0.7;

    println!("Generating mandelbrot...");

    let image = image::ImageBuffer::from_fn(image_width, image_height, |x, y| {
        let x_norm = x as f32 / image_width as f32;
        let y_norm = y as f32 / image_height as f32;
        let mut color = image::Rgb([(255.0 * x_norm) as u8, 0, (255.0 * y_norm) as u8]);
        let t = mandelbrot(scale * (x_norm - 0.5) + translate_x, scale * (y_norm - 0.5));
        color[1] = (255.0 * t) as u8;
        color
    });

    image.save("output.png").unwrap();
    println!("Image written to 'output.png'.");
}
