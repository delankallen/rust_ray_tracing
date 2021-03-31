use rust_ray_tracer::{camera::Camera, hittable::Hittable};
use rust_ray_tracer::vec3::Vec3;
use rust_ray_tracer::*;

use color::write_color;
use material::Material;
use vec3::*;
use sphere::Sphere;

use rand::prelude::*;

#[allow(unused)]
fn random_scene(rng:&mut impl Rng) -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Material::Lambertian {albedo: Vec3(0.5, 0.5, 0.5)};

    world.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0), 
        1000.0, 
        ground_material)
        ));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat:f32 = rng.gen();
            let center = Vec3((a as f32)+0.9*rng.gen::<f32>(), 0.2, (b as f32)+0.9*rng.gen::<f32>());

            if (center - Vec3(4.0,0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    world.push(Box::new(
                            Sphere::new(
                                center, 
                                0.2, 
                                Material::Lambertian {albedo}
                            )
                        )
                    )
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng,0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    world.push(Box::new(
                        Sphere::new(
                            center, 
                            0.2, 
                            Material::Metal {albedo, fuzz})
                        )
                    )
                } else {
                    world.push(Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Material::Dielectric {ir: 1.5})
                    ))
                }
            }
        }
    }

    world.push(Box::new(
        Sphere::new(
            Vec3(0.0, 1.0, 0.0), 
            1.0, 
        Material::Dielectric {ir: 1.5})
    ));
    world.push(Box::new(
        Sphere::new(
            Vec3(-4.0, 1.0, 0.0),
            1.0,
        Material::Lambertian {albedo: Vec3(0.4, 0.2, 0.1)})
    ));
    world.push(Box::new(
        Sphere::new(
            Vec3(4.0, 1.0, 0.0), 
            1.0, 
        Material::Metal {albedo: Vec3(0.7, 0.6, 0.5), fuzz:0.0})
    ));

    world
}

#[allow(unused)]
fn test_reflection() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Material::Lambertian {albedo: Vec3(0.4, 1.0, 0.7)};
    let material_center = Material::Lambertian {albedo: Vec3(0.7, 0.7, 1.0)};
    let material_left = Material::Dielectric {ir:1.5};
    let material_right = Material::Metal {albedo: Vec3(1.0,0.2,0.87), fuzz:0.0};

    world.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0), 
        100.0, 
        ground_material)
        ));
    world.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0), 
        0.5, 
        material_center)
        ));
    world.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0), 
        0.5, 
        material_left)
        ));
    world.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0), 
        0.5, 
        material_right)
        ));

    world
}


fn main() {
    //Image
    const ASPECT_RATIO: f32 = 3.0/2.0;
    let width = 200;
    let height= ((width as f32) / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i32 = 100;

    //World
    let mut rng = thread_rng();
    let world = random_scene(&mut rng);
    // let world = test_reflection();

    let lookfrom:Point3 = Vec3(13.0, 2.0, 3.0);
    let lookat:Point3 = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 20.0;
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
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (width as f32 -1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (height as f32-1.0);
                let r = cam.get_ray(u,v, &mut rng);
                pixel_color = pixel_color + ray_color(&world, r, 50,  &mut rng);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL as f32);
        }
    }

    eprintln!("\nDone.");
}
