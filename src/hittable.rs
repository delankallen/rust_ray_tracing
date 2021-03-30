use std::ops::Range;

use crate::{material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct HitRecord<'m> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'m Material,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable: std::fmt::Debug + Sync + Send {
    fn hit<'o>(
        &'o self, 
        ray: &Ray,
        t_range: Range<f32>,
        rng: &mut dyn FnMut() -> f32,
    ) -> Option<HitRecord<'o>>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit<'o>(
        &'o self, 
        ray: &Ray,
        t_range: Range<f32>,
        rng: &mut dyn FnMut() -> f32,
    ) -> Option<HitRecord<'o>> {
        (**self).hit(ray, t_range, rng)
    }
}