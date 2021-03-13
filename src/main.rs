
pub mod vec3;
pub mod color;
pub mod ray;

use color::write_color;
use vec3::{Color, Point3, Vec3};
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r:Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc,oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0*a);
    }

}

fn ray_color(r:Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0, -1.0));
        return 0.5 * Color::new(n.x()+1.0, n.y() + 1.0,n.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;

    //Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width, 0.0,0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0, focal_length);

    //Render


    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64)/((IMAGE_WIDTH-1) as f64);
            let v = (j as f64)/((IMAGE_HEIGHT-1) as f64);
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
