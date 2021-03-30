
pub mod sphere;
pub mod color;
pub mod hittable_list;
pub mod utility_funcs;
pub mod camera;
pub mod material;
pub mod hittable;
pub mod ray;
pub mod vec3;

use hittable_list::HittableList;
use material::scatter;
use rand::prelude::*;
use rayon::prelude::*;
use crate::ray::Ray;
use crate::vec3::*;

pub fn ray_color(world: &impl HittableList, mut ray: Ray, rng: &mut impl Rng) -> Color {
    let mut accumulator = Vec3::default();
    let mut strength = Vec3::from(1.0);

    let mut bounces = 0;

    while let Some(hit) = world.hit_top(&ray, rng) {
        accumulator = accumulator + strength;// * hit.material.emmitted(hit.p);
        if let Some((attenuation, scattered)) = scatter(&ray, &hit, rng) {
            ray = scattered;
            strength = strength * attenuation;            
        } else {
            return accumulator;
        }

        if bounces == 1 {
            return accumulator;
        }

        bounces += 1;
    }

    Vec3::default()
}

// fn ray_color(mut r: Ray, world: &impl HittableList, rng: &mut impl Rng) -> Color {
//     if depth <= 0 {return Vec3(0.0,0.0,0.0)}

//     if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
//         if let Some((attenuation, scattered)) = scatter( r, &rec) {
//             return attenuation * ray_color(&scattered, world, depth-1);
//         }

//         return Vec3(0.0,0.0,0.0);
//     }

//     let unit_direction = Vec3::unit_vector(r.direction());
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     return (1.0-t) * Vec3(1.0,1.0,1.0) + t * Vec3(0.5, 0.7, 1.0);
// }

pub struct Image(Vec<Vec<Vec3>>);

impl Image {
    pub fn par_compute(nx: usize, ny: usize, f: impl Fn(usize, usize) -> Vec3 + Sync) -> Image {
        Image(
            (0..ny)
                .into_par_iter()
                .rev()
                .map(|y| (0..nx).map(|x| f(x, y)).collect())
                .collect(),
        )
    }

    pub fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vec3) -> Image {
        Image(
            (0..ny)
                .rev()
                .map(|y| (0..nx).map(|x| f(x, y)).collect())
                .collect(),
        )
    }
}