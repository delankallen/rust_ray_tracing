const PI: f32 = 3.1415926535897932385;

pub fn clamp(x: f32, min: f32, max:f32) -> f32 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI/180.0
}