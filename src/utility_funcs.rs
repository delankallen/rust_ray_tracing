// const PI: f64 = 3.1415926535897932385;

pub fn clamp(x: f64, min: f64, max:f64) -> f64 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x
    }
}

// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * PI/180.0
// }