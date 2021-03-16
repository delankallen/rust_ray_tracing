use crate::{hittable::{HitRecord, Hittable}, ray::Ray, vec3::{Point3, Vec3}};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center:Point3, radius:f64) -> Self {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let oc:Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {return None;};
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd)/a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal:Vec3 = (r.at(root) - self.center) / self.radius;
        let f_face = Vec3::dot(r.direction(), outward_normal) < 0.0;

        return Some(HitRecord {
            t: root,
            p: r.at(root),
            normal: match f_face {
                true => outward_normal,
                false => -outward_normal
            },
            front_face: f_face
        });
    }
}