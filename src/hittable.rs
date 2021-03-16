use crate::{ray::Ray, vec3::{Point3, Vec3}};

#[derive(Debug,Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
}