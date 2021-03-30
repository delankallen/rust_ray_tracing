use std::ops::Range;

use crate::{hittable::{HitRecord, Hittable}, material::{Material}, ray::Ray, vec3::{Point3, Vec3}};

#[derive(Debug,Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center:Point3, radius:f32, material:Material) -> Self {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit<'o>(
        &'o self,
        r: &Ray,
        t_range: Range<f32>,
        _rng: &mut dyn FnMut() -> f32,
    ) -> Option<HitRecord> {
        let oc:Vec3 = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;        
        let sqrtd = discriminant.sqrt();

        if discriminant > 0.0 {
            for &t in &[
                (-half_b - sqrtd) / a,
                (-half_b + sqrtd) / a,
            ] {
                if t < t_range.end && t >= t_range.start {
                    let p = r.at(t);
                    let outward_normal:Vec3 = (r.at(t) - self.center) / self.radius;
                    return Some(HitRecord {
                        p,
                        normal: outward_normal,
                        material: &self.material,
                        t,
                        front_face: r.direction.dot(outward_normal) < 0.0,                        
                    })
                }
            }
            
        }
        None
    }
}