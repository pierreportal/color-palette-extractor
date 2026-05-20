use image::{DynamicImage, GenericImageView, Rgba};
use kmeans_colors::{Kmeans, get_kmeans};
use palette::Srgb;

fn load_image(filename: &str) -> DynamicImage {
    let img = image::open(filename).expect("failed to open image");
    img.resize(200, 200, image::imageops::FilterType::Triangle)
}

fn get_pixel_rgb(pixel: Rgba<u8>) -> Srgb<f32> {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];

    Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

fn convert_px_to_lab(img: DynamicImage, step: usize) -> Vec<Srgb> {
    let pixels: Vec<Srgb> = img
        .pixels()
        .step_by(step)
        .map(|(_, _, pixel)| get_pixel_rgb(pixel))
        .collect();
    pixels
}

fn rgb_to_hex(rgb: &Srgb) -> String {
    let r = (rgb.red * 255.0) as u8;
    let g = (rgb.green * 255.0) as u8;
    let b = (rgb.blue * 255.0) as u8;
    format!("#{r:02X}{g:02X}{b:02X}")
}

fn find_kmeans_centroids(buf: &Vec<Srgb>) -> Vec<Srgb> {
    let n_colors = 5;
    let max_iter = 20;
    let converge = 10.0;
    let seed = 42;
    let Kmeans { centroids, .. } = get_kmeans(n_colors, max_iter, converge, false, buf, seed);
    centroids
}

fn main() {
    let img = load_image("sample/img.jpg");
    let lab_buf = convert_px_to_lab(img, 4);
    let centroids = find_kmeans_centroids(&lab_buf);
    let palette: Vec<String> = centroids.iter().map(|lab| rgb_to_hex(lab)).collect();

    println!("Palette:");
    println!("{:?}", palette);
}
