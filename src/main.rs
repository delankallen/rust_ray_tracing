
pub mod vec3;
pub mod color;
pub mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod utility_funcs;
mod material;

use camera::Camera;
use color::write_color;
use hittable_list::HittableList;
use material::{Material, scatter};
use vec3::*;
use ray::Ray;
use hittable::Hittable;
use sphere::Sphere;

use rand::prelude::*;

fn random_scene() -> HittableList {
    let mut world = HittableList::new(Vec::new());

    let ground_material = Material::Lambertian {albedo: Color::new(0.5, 0.5, 0.5)};
    world.add(Box::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0), 
            1000.0, 
            ground_material)));
    
    for a in -11..11 {
        for b in -11..11 {
            let mut rng = thread_rng();
            let choose_mat:f64 = rng.gen();
            let center = Point3::new((a as f64)+0.9*rng.gen::<f64>(), 0.2, (b as f64)+0.9*rng.gen::<f64>());

            if (center - Point3::new(4.0,0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    world.add(Box::new(
                            Sphere::new(
                                center, 
                                0.2, 
                                Material::Lambertian {albedo}
                            )
                        )
                    )
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    world.add(Box::new(
                        Sphere::new(
                            center, 
                            0.2, 
                            Material::Metal {albedo, fuzz})
                        )
                    )
                } else {
                    world.add(Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Material::Dielectric {ir: 1.5})
                    ))
                }
            }
        }
    }

    world.add(Box::new(
        Sphere::new(
            Point3::new(0.0, 1.0, 0.0), 
            1.0, 
        Material::Dielectric {ir: 1.5})
    ));
    world.add(Box::new(
        Sphere::new(
            Point3::new(-4.0, 1.0, 0.0), 
            1.0, 
        Material::Lambertian {albedo: Color::new(0.4, 0.2, 0.1)})
    ));
    world.add(Box::new(
        Sphere::new(
            Point3::new(4.0, 1.0, 0.0), 
            1.0, 
        Material::Metal {albedo: Color::new(0.7, 0.6, 0.5), fuzz:0.0})
    ));

    world
}

fn ray_color(r:&Ray, world:&dyn Hittable, depth:i32) -> Color {
    if depth <= 0 {return Color::new(0.0,0.0,0.0)}

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = scatter( r, &rec) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }

        return Color::new(0.0,0.0,0.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 3.0/2.0;
    let width = 1200;
    let height= ((width as f64) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i32 = 500;
    const MAX_DEPTH:i32 = 50;

    //World
    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    //Camera

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    //Render

    println!("P3\n{} {}\n255", width, height);

    // for (r,g,b) in screen {
    //     println!("{} {} {}", r,g,b);
    // }

    for j in (0..height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..width {
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
                let r = cam.get_ray(u,v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as f64);
        }
    }

    eprintln!("\nDone.");
}
