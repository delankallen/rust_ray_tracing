use crate::vec3::Color;
use crate::utility_funcs::clamp;

pub fn write_color(pixel_color:Color, samples_per_pixel:f64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel;
    r*=scale;
    g*=scale;
    b*=scale;

    println!(
        "{} {} {}", 
        (256.0 * clamp(r, 0.0, 0.999)) as i32, 
        (256.0 * clamp(g, 0.0, 0.999)) as i32, 
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}