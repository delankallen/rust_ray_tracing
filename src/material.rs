use rand::Rng;

use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, Vec3, random_in_unit_sphere, random_unit_vector, reflect, refract}};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ir: f64}
}

pub fn scatter (
    ray_in: &Ray,
    rec: &HitRecord
) -> Option<(Color, Ray)> {
    match &rec.material {
        &Material::Lambertian { albedo } => {
            let mut scatter_direction = rec.normal + random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            };
    
            Some((albedo, Ray::new(rec.p, scatter_direction)))
        }
        &Material::Metal { albedo, fuzz} => {
            let f = if fuzz < 1.0 {fuzz} else {1.0};
            let reflected = reflect(Vec3::unit_vector(ray_in.direction()), rec.normal);
            let scattered = Ray::new(rec.p, reflected + f * random_in_unit_sphere());
    
            if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
                return Some((albedo,scattered))
            }
            None
        }
        &Material::Dielectric { ir } => {
            let mut rng = rand::thread_rng();
            let refraction_ratio = if rec.front_face {1.0/ir} else {ir};

            let unit_direction = Vec3::unit_vector(ray_in.direction());
            let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
            let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

            let direction = match refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > rng.gen() {
                true => reflect(unit_direction, rec.normal),
                false => refract(unit_direction, rec.normal, refraction_ratio)
            };

            Some((Color::new(1.0,1.0,1.0), Ray::new(rec.p, direction)))
        }
    }
}

fn reflectance(cosine:f64, ref_idx:f64) -> f64 {
    let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
    r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
}