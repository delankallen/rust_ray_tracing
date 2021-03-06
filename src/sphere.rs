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

        if discriminant < 0.0 {return None;};
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd)/a;
        if root < t_range.start || t_range.end < root {
            root = (-half_b + sqrtd) / a;
            if root < t_range.start || t_range.end < root {
                return None;
            }
        }

        let outward_normal:Vec3 = (r.at(root) - self.center) / self.radius;
        let f_face = r.direction.dot(outward_normal) < 0.0;
        // let material = self.material;

        return Some(HitRecord {
            t: root,
            p: r.at(root),
            normal: match f_face {
                true => outward_normal,
                false => -outward_normal
            },
            material: &self.material,
            front_face: f_face
        });
    }
}