use rand::{Rng, thread_rng};

use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, Vec3, reflect, refract}};

// cargo build --release && ./target/release/rust_ray_tracing.exe > image.ppm && emulsion image.ppm

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ir: f32}
}

impl Material {
    pub fn scatter (
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Color, Ray)> {
        match &rec.material {
            &Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.p + rec.normal + Vec3::random_unit_vector(rng);    
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                };

                let scatterd = Ray {
                    origin: rec.p,
                    direction: scatter_direction - rec.p
                };
        
                Some((*albedo, scatterd))
            }
            &Material::Metal { albedo, fuzz} => {
                let f = if *fuzz < 1.0 {*fuzz} else {1.0};
                let reflected = reflect(Vec3::unit_vector(ray_in.direction), rec.normal);
                let scattered = Ray::new(rec.p, reflected + f * Vec3::random_in_unit_sphere(rng));
        
                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some((*albedo,scattered))
                }
                None
            }
            &Material::Dielectric { ir } => {
                let mut rngg = thread_rng();
                let refraction_ratio = if rec.front_face {1.0/ *ir} else {*ir};
    
                let unit_direction = Vec3::unit_vector(ray_in.direction);
                let cos_theta = f32::min((-unit_direction).dot(rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
    
                let direction = match refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > rngg.gen() {
                    true => reflect(unit_direction, rec.normal),
                    false => refract(unit_direction, rec.normal, refraction_ratio)
                };

                // let direction = reflect(unit_direction, rec.normal);
    
                // let direction = refract(ray_in.direction, rec.normal, refraction_ratio)
                // .filter(|_| rng.gen::<f32>() >= reflectance(cos_theta, refraction_ratio))
                // .unwrap_or_else(|| reflect(ray_in.direction, rec.normal));
    
                Some((Vec3(1.0,1.0,1.0), Ray::new(rec.p, direction)))
            }
        }
    }

    pub fn emitted(&self, _p: Vec3) -> Vec3 {
        // match self {
        //     Material::DiffuseLight {
        //         emission,
        //         brightness,
        //     } => *brightness * emission(p),
        //     _ => Vec3::default(),
        // }

        Vec3::default()
    }

}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("...")
    }
}

#[inline]
fn reflectance(cosine:f32, ref_idx:f32) -> f32 {
    let r0 = (1.0-ref_idx) / (1.0+ref_idx);
    let r1 = r0*r0;
    r1 + (1.0 - r1)*(1.0 - cosine).powi(5)
}