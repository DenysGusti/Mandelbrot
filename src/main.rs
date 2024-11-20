mod client;
mod complex;
mod image;
mod mandelbrot;
mod tests;

fn main() {
    let (width, height, max_iterations) = client::parse_args().expect("Can't parse arguments");

    println!(
        "Generating Mandelbrot for {}x{} image (max_iterations: {})",
        width, height, max_iterations
    );

    let image = mandelbrot::generate_image(width, height, max_iterations);

    let mandelbrot_pixel_count = image.get_mandelbrot_pixels();

    println!("Pixels in the set: {}", mandelbrot_pixel_count);

    client::save_to_file(&image, "mandelbrot.ppm");
}
