
pub mod vec3;
pub mod color;
pub mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod utility_funcs;

use camera::Camera;
use color::write_color;
use hittable_list::HittableList;
use vec3::{Color, Point3, Vec3};
use ray::Ray;
use hittable::Hittable;
use sphere::Sphere;

use rand::prelude::*;

fn ray_color(r:&Ray, world:&dyn Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i32 = 100;

    //World
    let mut world = HittableList::new(Vec::new());

    world.add(Box::new(
        Sphere::new(
            Point3::new(0.0,0.0,-1.0),
            0.5
        )));
    world.add(Box::new(
        Sphere::new(
            Point3::new(0.0,-100.5,-1.0),
            100.0
        )));
    //Camera
    let cam = Camera::new();

    //Render


    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>())/(IMAGE_HEIGHT as f64 - 1.0);
                let r = cam.get_ray(u,v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as f64);
        }
    }

    eprintln!("\nDone.");
}
